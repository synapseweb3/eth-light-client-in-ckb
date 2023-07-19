//! Extensions for core types -- the essential rust types.

use ::core::result::Result;
use alloc::{vec, vec::Vec};

use ckb_mmr::{leaf_index_to_mmr_size, leaf_index_to_pos, Error as MMRError};
use rlp::encode;
use ssz_types::{typenum, FixedVector, VariableList};
use tree_hash::TreeHash as _;
use tree_hash_derive::TreeHash;

use crate::{
    consensus_specs::{self as specs, forks, helpers},
    error::{
        ClientBootstrapError, ClientUpdateError, SyncCommitteeUpdateError, TxVerificationError,
    },
    types::{core, packed, prelude::*},
    utilities::{bls, mmr, mpt, ssz},
};

impl core::Header {
    /// Initializes a client from a bootstrap header.
    pub fn initialize_client(&self) -> core::Client {
        let root = self.tree_hash_root();
        let digest = core::HeaderDigest {
            children_hash: root,
        };
        core::Client {
            id: 0,
            minimal_slot: self.slot,
            maximal_slot: self.slot,
            tip_header_root: root,
            headers_mmr_root: digest,
        }
    }
}

impl core::SyncAggregate {
    /// Checks if sync committee participation beyond supermajority (`>=2/3`, a two-thirds majority).
    ///
    /// References:
    /// - [Proof-Of-Stake (POS) / Finality](https://ethereum.org/en/developers/docs/consensus-mechanisms/pos/#finality)
    /// - [`is_better_update`](https://github.com/ethereum/consensus-specs/blob/v1.2.0/specs/altair/light-client/sync-protocol.md#is_better_update)
    /// - [`process_light_client_update`](https://github.com/ethereum/consensus-specs/blob/v1.2.0/specs/altair/light-client/sync-protocol.md#process_light_client_update)
    pub fn has_supermajority(&self) -> bool {
        let bits = self.sync_committee_bits.as_ref();
        let ones: usize = bits.iter().map(|byte| byte.count_ones() as usize).sum();
        let total = bits.len() * 8;
        debug!("check if is supermajority: {ones} / {total}");
        ones * 3 >= total * 2
    }
}

// The intermediate type of `SyncCommittee` which has SSZ support.
#[derive(TreeHash)]
struct SszSyncCommittee {
    pub pubkeys: FixedVector<core::BlsPubkey, typenum::U512>,
    pub aggregate_pubkey: core::BlsPubkey,
}

impl From<core::SyncCommittee> for SszSyncCommittee {
    fn from(data: core::SyncCommittee) -> Self {
        Self {
            pubkeys: FixedVector::from(data.pubkeys),
            aggregate_pubkey: data.aggregate_pubkey,
        }
    }
}

impl core::SyncCommittee {
    /// Calculates the tree hash root.
    pub fn tree_hash_root(self) -> core::Hash {
        SszSyncCommittee::from(self).tree_hash_root()
    }
}

impl core::ClientBootstrap {
    /// Verifies the corresponding packed client sync committee.
    ///
    /// References:
    /// - [`initialize_light_client_store`](https://github.com/ethereum/consensus-specs/blob/v1.2.0/specs/altair/light-client/sync-protocol.md#initialize_light_client_store)
    pub fn verify_packed_client_sync_committee(
        &self,
        packed_client_sync_committee: packed::ClientSyncCommitteeReader,
    ) -> Result<(), ClientBootstrapError> {
        //
        // Verify Self
        //

        if self.header.is_empty() {
            return Err(ClientBootstrapError::HeaderIsEmpty);
        }

        //
        // Verify Sync Committee
        //

        let client_sync_committee = packed_client_sync_committee.unpack();
        let expected_period = helpers::compute_sync_committee_period_at_slot(self.header.slot);
        if expected_period != client_sync_committee.period {
            warn!(
                "failed: period is expected to be {expected_period} \
                since slot is {}, but actual is {}",
                self.header.slot, client_sync_committee.period,
            );
            return Err(ClientBootstrapError::IncorrectPeriod);
        }
        if client_sync_committee.data.pubkeys.len() != forks::altair::SYNC_COMMITTEE_SIZE {
            error!(
                "failed: sync committee size is expected to be {}, but actual is {}.",
                forks::altair::SYNC_COMMITTEE_SIZE,
                client_sync_committee.data.pubkeys.len(),
            );
            return Err(ClientBootstrapError::UnexpectedSyncCommitteeSize);
        }
        let sync_committee_root = client_sync_committee.data.tree_hash_root();
        let (depth, index) =
            specs::get_depth_and_index_from_current_sync_committee_index(self.header.slot);
        if !ssz::is_valid_merkle_branch(
            &sync_committee_root,
            &self.current_sync_committee_branch,
            depth as usize,
            index,
            &self.header.state_root,
        ) {
            warn!(
                "failed: verify merkle branch for sync committee \
                (root: {sync_committee_root:#x}) with header {:#}, \
                depth: {depth}, index: {index}, branch_size: {}",
                self.header,
                self.current_sync_committee_branch.len(),
            );
            return Err(ClientBootstrapError::InvalidSyncCommitteeBranch);
        } else {
            debug!(
                "passed: verify merkle branch for sync committee \
                (root: {sync_committee_root:#x}) with header {:#}, \
                depth: {depth}, index: {index}, branch_size: {}",
                self.header,
                self.current_sync_committee_branch.len(),
            );
        }
        Ok(())
    }
}

impl core::ClientUpdate {
    /// Verifies the new client and the update that make it be upgrade from the old client.
    ///
    /// N.B. `client_sync_committee` should be checked.
    pub fn verify_client_update(
        &self,
        old_client: core::Client,
        genesis_validators_root: core::Hash,
        packed_client_sync_committee: packed::ClientSyncCommitteeReader,
        new_client: core::Client,
    ) -> Result<(), ClientUpdateError> {
        //
        // Verify Self
        //

        if self.attested_header.is_empty() {
            return Err(ClientUpdateError::AttestedHeaderIsEmpty);
        }
        if self.attested_header.slot >= self.signature_slot {
            error!(
                "failed: attested slot ({}) should be less than signature slot ({})",
                self.attested_header.slot, self.signature_slot
            );
            return Err(ClientUpdateError::BadSignatureSlot);
        }

        //
        // Verify Headers
        //

        let finalized_header = self.verify_headers_and_new_client(old_client, new_client)?;
        if finalized_header.is_empty() {
            return Err(ClientUpdateError::FinalizedHeaderIsEmpty);
        }

        //
        // Check Sync Committee
        //

        let client_sync_committee = packed_client_sync_committee.unpack();
        let signature_period = helpers::compute_sync_committee_period_at_slot(self.signature_slot);
        if client_sync_committee.period != signature_period {
            warn!(
                "failed: no matched client sync comittee, \
                signature period is {signature_period} (slot: {}), \
                but client sync committee period is {}",
                self.signature_slot, client_sync_committee.period,
            );
            return Err(ClientUpdateError::MismatchedSyncCommittee);
        }

        //
        // Verify the Signature with Current Sync Committee
        //

        if !self.sync_aggregate.has_supermajority() {
            return Err(ClientUpdateError::NotSupermajorityParticipation);
        }
        let attested_root = self.attested_header.tree_hash_root();
        let message = bls::compute_signing_root_at_signature_slot(
            attested_root,
            self.signature_slot,
            &forks::altair::DOMAIN_SYNC_COMMITTEE,
            genesis_validators_root,
        );
        let pubkeys = client_sync_committee
            .data
            .decompress_all_pubkeys()
            .map_err(|_| ClientUpdateError::BlsPublicKeyBytesError)?;
        let is_verified = self
            .sync_aggregate
            .fast_aggregate_verify(&pubkeys, message)
            .map_err(|_| ClientUpdateError::BlsAggregateSignatureError)?;
        if !is_verified {
            warn!(
                "failed: verify the signature for attested header, \
                signature slot: {}, current sync committee (period: {}), \
                attested root: {attested_root:#x}, \
                genesis validators root: {genesis_validators_root:#x}",
                self.signature_slot, client_sync_committee.period
            );
            return Err(ClientUpdateError::FailedToVerifyTheAttestedHeader);
        } else {
            debug!(
                "passed: verify the signature for attested header, \
                signature slot: {}, current sync committee (period: {}), \
                attested root: {attested_root:#x}, \
                genesis validators root: {genesis_validators_root:#x}",
                self.signature_slot, client_sync_committee.period
            );
        }

        //
        // Verify Finality Header
        //

        if finalized_header.inner.slot >= self.attested_header.slot {
            error!(
                "failed: finalized header ({}) should be after attested header ({})",
                finalized_header, self.attested_header
            );
            return Err(ClientUpdateError::FinalizedShouldBeAfterAttested);
        }
        let (depth, index) =
            specs::get_depth_and_index_from_finalized_root_index(self.attested_header.slot);
        if !ssz::is_valid_merkle_branch(
            &finalized_header.root,
            &self.finality_branch,
            depth as usize,
            index,
            &self.attested_header.state_root,
        ) {
            warn!(
                "failed: verify merkle branch for finalized header \
                {finalized_header} with attested header {:#}, \
                depth: {depth}, index: {index}, branch_size: {}",
                self.attested_header,
                self.finality_branch.len(),
            );
            return Err(ClientUpdateError::InvalidFinalityBranch);
        } else {
            debug!(
                "passed: verify merkle branch for finalized header \
                {finalized_header} with attested header {:#}, \
                depth: {depth}, index: {index}, branch_size: {}",
                self.attested_header,
                self.finality_branch.len(),
            );
        }
        Ok(())
    }

    fn verify_headers_and_new_client(
        &self,
        client: core::Client,
        new_client: core::Client,
    ) -> Result<mmr::HeaderWithCache, ClientUpdateError> {
        if self.headers.is_empty() {
            error!("failed: client update has no headers");
            return Err(ClientUpdateError::EmptyHeaders);
        }

        let headers_count = self.headers.len();
        info!(
            "update client with headers (len: {headers_count}), \
            client: {client}, new client: {new_client}"
        );

        let mut headers_iter = self.headers.iter();
        let mut curr_cached_header = headers_iter.next().cloned().unwrap().calc_cache();
        let mut prev_cached_header: mmr::HeaderWithCache;
        let mut curr_tip_valid_header_root: core::Hash;
        let mut header_mmr_index: u64;
        let mut digests = Vec::with_capacity(headers_count);

        // Check First Header with the Old Client
        {
            debug!("first header: {curr_cached_header}");

            // Check Old Tip Header (with the first header)
            if curr_cached_header.inner.slot != client.maximal_slot + 1 {
                error!(
                    "first header isn't continuous with client on slots, \
                     client: {client}, header: {curr_cached_header}"
                );
                return Err(ClientUpdateError::FirstHeaderSlot);
            }

            curr_tip_valid_header_root = if curr_cached_header.inner.is_empty() {
                client.tip_header_root
            } else {
                if curr_cached_header.inner.parent_root != client.tip_header_root {
                    error!(
                        "first header isn't continuous with client on parent root, \
                         client: {client}, header: {curr_cached_header}"
                    );
                    return Err(ClientUpdateError::FirstHeaderParentRoot);
                }
                curr_cached_header.root
            };

            header_mmr_index = curr_cached_header.inner.slot - client.minimal_slot;

            trace!(
                "first header (slot: {}) in MMR on index {header_mmr_index}",
                curr_cached_header.inner.slot
            );
            let digest = curr_cached_header.packed_digest();
            digests.push(digest);

            header_mmr_index += 1;
            prev_cached_header = curr_cached_header;
        }

        // Check if headers are continuous
        for header in headers_iter {
            curr_cached_header = header.clone().calc_cache();

            trace!(
                "current valid header root: {curr_tip_valid_header_root:#x}, \
                current header: {curr_cached_header}"
            );

            if prev_cached_header.inner.slot + 1 != curr_cached_header.inner.slot {
                error!(
                    "current header isn't continuous with previous header on slots, \
                    current: {curr_cached_header}, previous: {prev_cached_header}"
                );
                return Err(ClientUpdateError::UncontinuousSlot);
            }

            if !curr_cached_header.inner.is_empty() {
                if curr_tip_valid_header_root != curr_cached_header.inner.parent_root {
                    error!(
                        "current header isn't continuous with previous header on root, \
                        current tip valid header root: {curr_tip_valid_header_root:#x}, \
                        current: {curr_cached_header}, previous: {prev_cached_header}"
                    );
                    return Err(ClientUpdateError::UnmatchedParentRoot);
                }
                curr_tip_valid_header_root = curr_cached_header.root;
            }

            trace!(
                "current header (slot: {}) in MMR on index {header_mmr_index}",
                curr_cached_header.inner.slot
            );
            let digest = curr_cached_header.packed_digest();
            digests.push(digest);

            header_mmr_index += 1;
            prev_cached_header = curr_cached_header;
        }

        let new_maximal_slot = prev_cached_header.inner.slot;

        // Check MMR Root
        {
            let proof: mmr::MMRProof = {
                let max_index = new_maximal_slot - client.minimal_slot;
                let mmr_size = leaf_index_to_mmr_size(max_index);
                debug!("check MMR root with size: {mmr_size}, max-index: {max_index}");
                // TODO optimize
                let proof = self
                    .new_headers_mmr_proof
                    .iter()
                    .map(|item| item.pack())
                    .collect::<Vec<_>>();
                mmr::MMRProof::new(mmr_size, proof)
            };
            let result = proof
                .verify_incremental(
                    new_client.headers_mmr_root.pack(),
                    client.headers_mmr_root.pack(),
                    digests,
                )
                .map_err(|_| ClientUpdateError::MmrError)?;
            if !result {
                warn!(
                    "failed: verify MMR proof for headers between {} and {new_maximal_slot}",
                    client.maximal_slot + 1
                );
                return Err(ClientUpdateError::HeadersMmrProof);
            } else {
                debug!(
                    "passed: verify MMR proof for headers between {} and {new_maximal_slot}",
                    client.maximal_slot + 1
                );
            }
        }

        // Check New Client
        if new_client.id != client.id {
            error!(
                "failed: new client id has been changed ({} -> {})",
                client.id, new_client.id
            );
            return Err(ClientUpdateError::ClientIdChanged);
        }
        if new_client.minimal_slot != client.minimal_slot {
            error!(
                "failed: new client minimal slot has been changed ({} -> {})",
                client.minimal_slot, new_client.minimal_slot
            );
            return Err(ClientUpdateError::ClientMinimalSlotChanged);
        }
        if new_client.maximal_slot != new_maximal_slot {
            error!(
                "failed: new client maximal slot ({}) is incorrect, expect {new_maximal_slot}",
                new_client.maximal_slot
            );
            return Err(ClientUpdateError::ClientMaximalSlot);
        }
        if new_client.tip_header_root != curr_tip_valid_header_root {
            error!(
                "failed: new client tip valid header root ({:#x}) is incorrect, \
                expect {curr_tip_valid_header_root}",
                new_client.tip_header_root
            );
            return Err(ClientUpdateError::ClientTipHeaderRoot);
        }

        Ok(prev_cached_header)
    }
}

impl core::SyncCommitteeUpdate {
    /// Verifies the packed next client sync committee with maximal slot in the last client and
    /// the packed current client sync committee.
    ///
    /// N.B. `current_client_sync_committee` should be checked.
    pub fn verify_packed_client_sync_committee(
        &self,
        maximal_slot_in_last_client: u64,
        genesis_validators_root: core::Hash,
        packed_current_client_sync_committee: packed::ClientSyncCommitteeReader,
        packed_next_client_sync_committee: packed::ClientSyncCommitteeReader,
    ) -> Result<(), SyncCommitteeUpdateError> {
        //
        // Verify Self
        //

        if self.attested_header.is_empty() {
            return Err(SyncCommitteeUpdateError::AttestedHeaderIsEmpty);
        }
        if self.attested_header.slot >= self.signature_slot {
            error!(
                "failed: attested slot ({}) should be less than signature slot ({})",
                self.attested_header.slot, self.signature_slot
            );
            return Err(SyncCommitteeUpdateError::BadSignatureSlot);
        }

        //
        // Check Current Sync Committee
        //

        let last_client_period =
            helpers::compute_sync_committee_period_at_slot(maximal_slot_in_last_client);
        let current_client_sync_committee = packed_current_client_sync_committee.unpack();
        if current_client_sync_committee.period != last_client_period {
            error!(
                "failed: current client sync committee period ({}) is not same as \
                the maximal period ({}) in the last client (slot: {maximal_slot_in_last_client})",
                current_client_sync_committee.period, last_client_period
            );
            return Err(SyncCommitteeUpdateError::BadCurrentPeriod);
        }
        let signature_period = helpers::compute_sync_committee_period_at_slot(self.signature_slot);
        // TODO Confirmation Required: if next sync committee could be signed in next period?
        if current_client_sync_committee.period != signature_period {
            warn!(
                "failed: signature (slot: {}, period: {signature_period}) \
                could NOT be verified with current sync committee (period: {})",
                self.signature_slot, current_client_sync_committee.period
            );
            return Err(SyncCommitteeUpdateError::SignatureInNextPeriod);
        }

        //
        // Verify the Signature with Current Sync Committee
        //

        if !self.sync_aggregate.has_supermajority() {
            return Err(SyncCommitteeUpdateError::NotSupermajorityParticipation);
        }
        let attested_root = self.attested_header.tree_hash_root();
        let message = bls::compute_signing_root_at_signature_slot(
            attested_root,
            self.signature_slot,
            &forks::altair::DOMAIN_SYNC_COMMITTEE,
            genesis_validators_root,
        );
        let pubkeys = current_client_sync_committee
            .data
            .decompress_all_pubkeys()
            .map_err(|_| SyncCommitteeUpdateError::BlsPublicKeyBytesError)?;
        let is_verified = self
            .sync_aggregate
            .fast_aggregate_verify(&pubkeys, message)
            .map_err(|_| SyncCommitteeUpdateError::BlsAggregateSignatureError)?;
        if !is_verified {
            warn!(
                "failed: verify the signature for attested header, \
                signature slot: {}, current sync committee (period: {}), \
                attested root: {attested_root:#x}, \
                genesis validators_root: {genesis_validators_root:#x}",
                self.signature_slot, current_client_sync_committee.period
            );
            return Err(SyncCommitteeUpdateError::FailedToVerifyTheAttestedHeader);
        } else {
            debug!(
                "passed: verify the signature for attested header, \
                signature slot: {}, current sync committee (period: {}), \
                attested root: {attested_root:#x}, \
                genesis validators_root: {genesis_validators_root:#x}",
                self.signature_slot, current_client_sync_committee.period
            );
        }

        //
        // Verify Next Sync Committee
        //

        let next_client_sync_committee = packed_next_client_sync_committee.unpack();
        if current_client_sync_committee.period + 1 != next_client_sync_committee.period {
            error!(
                "failed: periods are not continuous (current: {}, next: {})",
                current_client_sync_committee.period, next_client_sync_committee.period
            );
            return Err(SyncCommitteeUpdateError::NoncontinuousPeriods);
        }
        if next_client_sync_committee.data.pubkeys.len() != forks::altair::SYNC_COMMITTEE_SIZE {
            error!(
                "failed: next sync committee size is expected to be {}, but actual is {}.",
                forks::altair::SYNC_COMMITTEE_SIZE,
                next_client_sync_committee.data.pubkeys.len(),
            );
            return Err(SyncCommitteeUpdateError::UnexpectedNextSyncCommitteeSize);
        }
        let next_sync_committee_root = next_client_sync_committee.data.tree_hash_root();
        let (depth, index) =
            specs::get_depth_and_index_from_next_sync_committee_index(self.attested_header.slot);
        if !ssz::is_valid_merkle_branch(
            &next_sync_committee_root,
            &self.next_sync_committee_branch,
            depth as usize,
            index,
            &self.attested_header.state_root,
        ) {
            warn!(
                "failed: verify merkle branch for next sync committee \
                (root: {next_sync_committee_root:#x}) with attested header {:#}, \
                depth: {depth}, index: {index}, branch_size: {}",
                self.attested_header,
                self.next_sync_committee_branch.len(),
            );
            return Err(SyncCommitteeUpdateError::InvalidNextSyncCommitteeBranch);
        } else {
            debug!(
                "passed: verify merkle branch for next sync committee \
                (root: {next_sync_committee_root:#x}) with attested header {:#}, \
                depth: {depth}, index: {index}, branch_size: {}",
                self.attested_header,
                self.next_sync_committee_branch.len(),
            );
        }
        Ok(())
    }
}

impl core::Client {
    /// Verifies the corresponding transaction that in the proof is in the chain.
    pub fn verify_packed_transaction_proof(
        &self,
        tx_proof: packed::TransactionProofReader,
    ) -> Result<(), TxVerificationError> {
        let header_slot = tx_proof.header().slot().unpack();
        if self.minimal_slot > header_slot || self.maximal_slot < header_slot {
            log_if_enabled!(|Warn| {
                let tx_proof = tx_proof.unpack();
                let header = tx_proof.header.calc_cache();
                warn!(
                    "failed: verify slots for header {:#x}, for its {}-th transaction \
                    (client: [{}, {}], header-slot: {header_slot})",
                    header.root, tx_proof.transaction_index, self.minimal_slot, self.maximal_slot
                );
            });
            return Err(TxVerificationError::Unsynchronized);
        }
        let result = self
            .verify_single_header(tx_proof.header(), tx_proof.header_mmr_proof())
            .map_err(|_| TxVerificationError::MmrError)?;
        if !result {
            log_if_enabled!(|Warn| {
                let tx_proof = tx_proof.unpack();
                let header = tx_proof.header.calc_cache();
                warn!(
                    "failed: verify MMR proof for header {:#x}, for its {}-th transaction",
                    header.root, tx_proof.transaction_index
                );
            });
            Err(TxVerificationError::HeaderMmrProof)
        } else {
            log_if_enabled!(|Debug| {
                let tx_proof = tx_proof.unpack();
                let header = tx_proof.header.calc_cache();
                debug!(
                    "passed: verify MMR proof for header {:#x}, for its {}-th transaction",
                    header.root, tx_proof.transaction_index
                );
            });
            Ok(())
        }
    }

    /// Verifies the MMR proof that proves the corresponding header is in the chain.
    pub fn verify_single_header(
        &self,
        header: packed::HeaderReader,
        header_mmr_proof: packed::MmrProofReader,
    ) -> Result<bool, MMRError> {
        let header_slot = header.slot().unpack();
        let proof: mmr::MMRProof = {
            let max_index = self.maximal_slot - self.minimal_slot;
            let mmr_size = leaf_index_to_mmr_size(max_index);
            trace!(
                "verify MMR proof for header#{header_slot} with \
                MMR {{ size: {mmr_size}, max-index: {max_index} }}"
            );
            let proof = header_mmr_proof
                .iter()
                .map(|r| r.to_entity())
                .collect::<Vec<_>>();
            mmr::MMRProof::new(mmr_size, proof)
        };
        let digests_with_positions = {
            let index = header_slot - self.minimal_slot;
            let position = leaf_index_to_pos(index);
            let header_with_cache = header.unpack().calc_cache();
            trace!(
                "verify MMR proof for header#{header_slot} with \
                index: {index}, position: {position}, root: {:#x}",
                header_with_cache.root
            );
            let digest = header_with_cache.packed_digest();
            vec![(position, digest)]
        };
        proof.verify(self.headers_mmr_root.pack(), digests_with_positions)
    }
}

impl core::TransactionProof {
    /// Verifies raw bytes of the corresponding transaction and its receipt.
    pub fn verify_packed_payload(
        &self,
        payload: packed::TransactionPayloadReader,
    ) -> Result<(), TxVerificationError> {
        self.verify_transaction(payload.transaction().raw_data())?;
        self.verify_receipt(payload.receipt().raw_data())
    }

    /// Verifies raw bytes of the corresponding transaction.
    pub fn verify_transaction(&self, transaction: &[u8]) -> Result<(), TxVerificationError> {
        // Since `MAX_BYTES_PER_TRANSACTION`.
        // Ref: https://github.com/ethereum/consensus-specs/blob/v1.2.0/specs/bellatrix/beacon-chain.md#execution
        VariableList::<u8, typenum::U1073741824>::new(transaction.to_vec())
            .map_err(|_| TxVerificationError::SszError)
            .and_then(|tx| {
                let tx_root = tx.tree_hash_root();
                let tx_index = self.transaction_index as usize;
                let tx_in_block_offset =
                    specs::get_generalized_index_of_transaction_in_block_body_offset(
                        self.header.slot,
                    );
                let tx_in_block_index = tx_index + tx_in_block_offset;
                if !ssz::verify_merkle_proof(
                    &self.header.body_root,
                    &tx_root,
                    &self.transaction_ssz_proof,
                    tx_in_block_index,
                ) {
                    warn!(
                        "failed: verify SSZ proof for transaction {tx_root:#x} \
                        (index: {tx_index}, offset: {tx_in_block_index})"
                    );
                    Err(TxVerificationError::TransactionSszProof)
                } else {
                    debug!(
                        "passed: verify SSZ proof for transaction {tx_root:#x} \
                        (index: {tx_index}, offset: {tx_in_block_index})"
                    );
                    Ok(())
                }
            })
    }

    /// Verifies raw bytes of the corresponding transaction receipt.
    pub fn verify_receipt(&self, receipt: &[u8]) -> Result<(), TxVerificationError> {
        let key = encode(&self.transaction_index);
        let receipts_root_in_block_body =
            specs::get_generalized_index_of_receipts_root_in_block_body(self.header.slot);
        if !mpt::verify_proof(
            &self.receipt_mpt_proof,
            self.receipts_root.as_bytes(),
            &key,
            receipt,
        ) {
            warn!(
                "failed: verify MPT proof for {}-th receipt with root {:#x}",
                self.transaction_index, self.receipts_root
            );
            Err(TxVerificationError::ReceiptMptProof)
        } else if !ssz::verify_merkle_proof(
            &self.header.body_root,
            &self.receipts_root,
            &self.receipts_root_ssz_proof,
            receipts_root_in_block_body,
        ) {
            warn!(
                "failed: verify SSZ proof for {}-th receipt with root {:#x}",
                self.transaction_index, self.receipts_root
            );
            Err(TxVerificationError::ReceiptsRootSszProof)
        } else {
            debug!(
                "passed: verify MPT & SSZ proofs for {}-th receipt with root {:#x}",
                self.transaction_index, self.receipts_root
            );
            Ok(())
        }
    }
}
