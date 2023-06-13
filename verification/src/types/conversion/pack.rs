use bls::{PUBLIC_KEY_BYTES_LEN, SIGNATURE_BYTES_LEN};
use molecule::prelude::*;

use crate::types::{core, packed, prelude::Pack};

impl Pack<packed::Uint64> for core::Uint64 {
    fn pack(&self) -> packed::Uint64 {
        let data = molecule::bytes::Bytes::from(self.to_le_bytes().to_vec());
        packed::Uint64::new_unchecked(data)
    }
}

impl Pack<packed::Hash> for core::Hash {
    fn pack(&self) -> packed::Hash {
        let data = molecule::bytes::Bytes::from(self.as_bytes().to_vec());
        packed::Hash::new_unchecked(data)
    }
}

impl Pack<packed::Bytes> for core::Bytes {
    fn pack(&self) -> packed::Bytes {
        let len = self.len();
        let mut vec: Vec<u8> = Vec::with_capacity(molecule::NUMBER_SIZE + len);
        let len_bytes = molecule::pack_number(len as molecule::Number);
        vec.extend_from_slice(&len_bytes);
        vec.extend_from_slice(self);
        let data = molecule::bytes::Bytes::from(vec);
        packed::Bytes::new_unchecked(data)
    }
}

impl Pack<packed::SszProof> for core::SszProof {
    fn pack(&self) -> packed::SszProof {
        packed::SszProof::new_builder()
            .set(self.iter().map(|v| v.pack()).collect())
            .build()
    }
}

impl Pack<packed::MptProof> for core::MptProof {
    fn pack(&self) -> packed::MptProof {
        packed::MptProof::new_builder()
            .set(self.iter().map(|v| v.pack()).collect())
            .build()
    }
}

impl Pack<packed::HeaderDigest> for core::HeaderDigest {
    fn pack(&self) -> packed::HeaderDigest {
        packed::HeaderDigest::new_builder()
            .children_hash(self.children_hash.pack())
            .build()
    }
}

impl Pack<packed::MmrProof> for core::MmrProof {
    fn pack(&self) -> packed::MmrProof {
        packed::MmrProof::new_builder()
            .set(self.iter().map(|v| v.pack()).collect())
            .build()
    }
}

impl Pack<packed::BlsPubkey> for core::BlsPubkey {
    fn pack(&self) -> packed::BlsPubkey {
        assert_eq!(self.as_serialized().len(), PUBLIC_KEY_BYTES_LEN);
        let data = molecule::bytes::Bytes::from(self.as_serialized().to_vec());
        packed::BlsPubkey::new_unchecked(data)
    }
}

impl Pack<packed::BlsSignature> for core::BlsSignature {
    fn pack(&self) -> packed::BlsSignature {
        assert_eq!(self.as_ref().len(), SIGNATURE_BYTES_LEN);
        let data = molecule::bytes::Bytes::from(self.as_ref().to_vec());
        packed::BlsSignature::new_unchecked(data)
    }
}

impl Pack<packed::BlsPubkeyVec> for core::BlsPubkeyVec {
    fn pack(&self) -> packed::BlsPubkeyVec {
        packed::BlsPubkeyVec::new_builder()
            .set(self.iter().map(|v| v.pack()).collect())
            .build()
    }
}

impl Pack<packed::Header> for core::Header {
    fn pack(&self) -> packed::Header {
        packed::Header::new_builder()
            .slot(self.slot.pack())
            .proposer_index(self.proposer_index.pack())
            .parent_root(self.parent_root.pack())
            .state_root(self.state_root.pack())
            .body_root(self.body_root.pack())
            .build()
    }
}

impl Pack<packed::HeaderVec> for core::HeaderVec {
    fn pack(&self) -> packed::HeaderVec {
        packed::HeaderVec::new_builder()
            .set(self.iter().map(|v| v.pack()).collect())
            .build()
    }
}

impl Pack<packed::SyncCommitteeBits> for core::SyncCommitteeBits {
    fn pack(&self) -> packed::SyncCommitteeBits {
        let data = molecule::bytes::Bytes::from(self.as_bytes().to_vec());
        packed::SyncCommitteeBits::new_unchecked(data)
    }
}

impl Pack<packed::SyncAggregate> for core::SyncAggregate {
    fn pack(&self) -> packed::SyncAggregate {
        packed::SyncAggregate::new_builder()
            .sync_committee_bits(self.sync_committee_bits.pack())
            .sync_committee_signature(self.sync_committee_signature.pack())
            .build()
    }
}

impl Pack<packed::SyncCommittee> for core::SyncCommittee {
    fn pack(&self) -> packed::SyncCommittee {
        packed::SyncCommittee::new_builder()
            .pubkeys(self.pubkeys.pack())
            .aggregate_pubkey(self.aggregate_pubkey.pack())
            .build()
    }
}

impl Pack<packed::ClientBootstrap> for core::ClientBootstrap {
    fn pack(&self) -> packed::ClientBootstrap {
        packed::ClientBootstrap::new_builder()
            .header(self.header.pack())
            .current_sync_committee_branch(self.current_sync_committee_branch.pack())
            .build()
    }
}

impl Pack<packed::ClientUpdate> for core::ClientUpdate {
    fn pack(&self) -> packed::ClientUpdate {
        packed::ClientUpdate::new_builder()
            .attested_header(self.attested_header.pack())
            .finality_branch(self.finality_branch.pack())
            .sync_aggregate(self.sync_aggregate.pack())
            .signature_slot(self.signature_slot.pack())
            .new_headers_mmr_proof(self.new_headers_mmr_proof.pack())
            .headers(self.headers.pack())
            .build()
    }
}

impl Pack<packed::SyncCommitteeUpdate> for core::SyncCommitteeUpdate {
    fn pack(&self) -> packed::SyncCommitteeUpdate {
        packed::SyncCommitteeUpdate::new_builder()
            .attested_header(self.attested_header.pack())
            .next_sync_committee_branch(self.next_sync_committee_branch.pack())
            .sync_aggregate(self.sync_aggregate.pack())
            .signature_slot(self.signature_slot.pack())
            .build()
    }
}

impl Pack<packed::TransactionProof> for core::TransactionProof {
    fn pack(&self) -> packed::TransactionProof {
        packed::TransactionProof::new_builder()
            .header(self.header.pack())
            .transaction_index(self.transaction_index.pack())
            .receipts_root(self.receipts_root.pack())
            .header_mmr_proof(self.header_mmr_proof.pack())
            .transaction_ssz_proof(self.transaction_ssz_proof.pack())
            .receipt_mpt_proof(self.receipt_mpt_proof.pack())
            .receipts_root_ssz_proof(self.receipts_root_ssz_proof.pack())
            .build()
    }
}

impl Pack<packed::TransactionPayload> for core::TransactionPayload {
    fn pack(&self) -> packed::TransactionPayload {
        packed::TransactionPayload::new_builder()
            .transaction(self.transaction.pack())
            .receipt(self.receipt.pack())
            .build()
    }
}

impl Pack<packed::ClientInfo> for core::ClientInfo {
    fn pack(&self) -> packed::ClientInfo {
        packed::ClientInfo::new_builder()
            .last_client_id(self.last_client_id.into())
            .minimal_headers_count(self.minimal_headers_count.into())
            .genesis_validators_root(self.genesis_validators_root.pack())
            .build()
    }
}

impl Pack<packed::Client> for core::Client {
    fn pack(&self) -> packed::Client {
        packed::Client::new_builder()
            .id(self.id.into())
            .minimal_slot(self.minimal_slot.pack())
            .maximal_slot(self.maximal_slot.pack())
            .tip_header_root(self.tip_header_root.pack())
            .headers_mmr_root(self.headers_mmr_root.pack())
            .build()
    }
}

impl Pack<packed::ClientSyncCommittee> for core::ClientSyncCommittee {
    fn pack(&self) -> packed::ClientSyncCommittee {
        packed::ClientSyncCommittee::new_builder()
            .period(self.period.pack())
            .data(self.data.pack())
            .build()
    }
}

impl Pack<packed::ClientTypeArgs> for core::ClientTypeArgs {
    fn pack(&self) -> packed::ClientTypeArgs {
        packed::ClientTypeArgs::new_builder()
            .type_id(self.type_id.pack())
            .clients_count(self.clients_count.into())
            .build()
    }
}
