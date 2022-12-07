pub mod core;
mod generated;
pub mod prelude;

mod conversion;

use ::core::result::Result;
use alloc::{vec, vec::Vec};

use ckb_mmr::{leaf_index_to_mmr_size, leaf_index_to_pos};
use rlp::encode;
use ssz_types::{typenum, VariableList};
use tree_hash::TreeHash as _;

pub use generated::packed;

use self::prelude::*;
use crate::{constants::generalized_index_offsets, mmr, ssz, trie};

impl core::Client {
    pub fn verify_packed_transaction_proof(
        &self,
        tx_proof: packed::TransactionProofReader,
    ) -> bool {
        let header_slot = tx_proof.header().slot().unpack();
        if self.minimal_slot > header_slot || self.maximal_slot < header_slot {
            return false;
        }
        self.verify_single_header(tx_proof.header(), tx_proof.header_mmr_proof())
    }

    #[allow(clippy::result_unit_err)] // TODO fix clippy
    pub fn try_apply_packed_updates(
        &self,
        packed_updates: packed::HeadersUpdateReader,
    ) -> Result<Self, ()> {
        // Simplest Checks
        if packed_updates.headers().is_empty()
            || packed_updates.updates().is_empty()
            || packed_updates.headers().len() != packed_updates.updates().len()
        {
            return Err(());
        }

        // Check Tip header
        {
            let tip_header_slot = packed_updates.tip_header().slot().unpack();
            if tip_header_slot != self.maximal_slot {
                return Err(());
            }
            let result = self.verify_single_header(
                packed_updates.tip_header(),
                packed_updates.tip_header_mmr_proof(),
            );
            if !result {
                return Err(());
            }
        }

        let headers = packed_updates.headers().unpack();
        let end_slot = headers[headers.len() - 1].slot;

        // Check Updates
        {
            let start_slot = headers[0].slot;

            if start_slot != self.maximal_slot + 1 {
                return Err(());
            }

            for pair in headers.windows(2) {
                if pair[0].slot + 1 != pair[1].slot {
                    return Err(());
                }
            }

            // TODO verify BLS pubkeys
        };

        // Check New MMR Root
        {
            let proof: mmr::MMRProof = {
                let max_index = end_slot - self.minimal_slot;
                let mmr_size = leaf_index_to_mmr_size(max_index);
                let proof = packed_updates
                    .new_headers_mmr_proof()
                    .iter()
                    .map(|r| r.to_entity())
                    .collect::<Vec<_>>();
                mmr::MMRProof::new(mmr_size, proof)
            };
            let digests_with_positions = packed_updates
                .headers()
                .iter()
                .map(|header| {
                    let header_slot = header.slot().unpack();
                    let index = header_slot - self.minimal_slot;
                    let position = leaf_index_to_pos(index);
                    let digest = header.digest();
                    (position, digest)
                })
                .collect::<Vec<_>>();
            let result = proof
                .verify(
                    packed_updates.new_headers_mmr_root().to_entity(),
                    digests_with_positions,
                )
                .unwrap_or(false);
            if !result {
                return Err(());
            }
        }

        let new_client = Self {
            minimal_slot: self.minimal_slot,
            maximal_slot: end_slot,
            headers_mmr_root: packed_updates.new_headers_mmr_root().unpack(),
        };

        Ok(new_client)
    }

    fn verify_single_header(
        &self,
        header: packed::Eth2HeaderReader,
        header_mmr_proof: packed::MmrProofReader,
    ) -> bool {
        let header_slot = header.slot().unpack();
        let proof: mmr::MMRProof = {
            let max_index = self.maximal_slot - self.minimal_slot;
            let mmr_size = leaf_index_to_mmr_size(max_index);
            let proof = header_mmr_proof
                .iter()
                .map(|r| r.to_entity())
                .collect::<Vec<_>>();
            mmr::MMRProof::new(mmr_size, proof)
        };
        let digests_with_positions = {
            let index = header_slot - self.minimal_slot;
            let position = leaf_index_to_pos(index);
            let digest = header.digest();
            vec![(position, digest)]
        };
        proof
            .verify(self.headers_mmr_root.pack(), digests_with_positions)
            .unwrap_or(false)
    }
}

impl core::TransactionProof {
    pub fn verify_packed_payload(&self, payload: packed::TransactionPayloadReader) -> bool {
        if !self.verify_transaction(payload.transaction().raw_data()) {
            return false;
        }
        self.verify_receipt(payload.receipt().raw_data())
    }

    pub fn verify_transaction(&self, transaction: &[u8]) -> bool {
        // Since `MAX_BYTES_PER_TRANSACTION`.
        VariableList::<u8, typenum::U1073741824>::new(transaction.to_vec())
            .map(|tx| {
                ssz::verify_merkle_proof(
                    self.header.body_root,
                    tx.tree_hash_root(),
                    &self.transaction_ssz_proof,
                    self.transaction_index as usize
                        + generalized_index_offsets::TRANSACTION_IN_BLOCK_BODY,
                )
            })
            .unwrap_or(false)
    }

    pub fn verify_receipt(&self, receipt: &[u8]) -> bool {
        let key = encode(&self.transaction_index);
        if !trie::verify_proof(
            &self.receipt_mpt_proof,
            self.receipts_root.as_bytes(),
            &key,
            receipt,
        ) {
            return false;
        }
        ssz::verify_merkle_proof(
            self.header.body_root,
            self.receipts_root,
            &self.receipts_root_ssz_proof,
            generalized_index_offsets::RECEIPTS_ROOT_IN_BLOCK_BODY,
        )
    }
}
