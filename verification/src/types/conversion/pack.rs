use molecule::prelude::*;

use crate::types::{core, packed, prelude::*};

impl Pack<packed::Uint64> for core::Uint64 {
    fn pack(&self) -> packed::Uint64 {
        packed::Uint64::new_unchecked(molecule::bytes::Bytes::from(self.to_le_bytes().to_vec()))
    }
}

impl Pack<packed::Hash> for core::Hash {
    fn pack(&self) -> packed::Hash {
        packed::Hash::new_unchecked(molecule::bytes::Bytes::from(self.as_bytes().to_vec()))
    }
}

impl Pack<packed::Bytes> for core::Bytes {
    fn pack(&self) -> packed::Bytes {
        let len = self.len();
        let mut vec: Vec<u8> = Vec::with_capacity(4 + len);
        vec.extend_from_slice(&(len as u32).to_le_bytes()[..]);
        vec.extend_from_slice(self);
        packed::Bytes::new_unchecked(molecule::bytes::Bytes::from(vec))
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

impl Pack<packed::ProofUpdate> for core::ProofUpdate {
    fn pack(&self) -> packed::ProofUpdate {
        packed::ProofUpdate::new_builder()
            .new_headers_mmr_root(self.new_headers_mmr_root.pack())
            .new_headers_mmr_proof(self.new_headers_mmr_proof.pack())
            .updates(self.updates.pack())
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
            .last_id(self.last_id.into())
            .minimal_updates_count(self.minimal_updates_count.into())
            .build()
    }
}

impl Pack<packed::Client> for core::Client {
    fn pack(&self) -> packed::Client {
        packed::Client::new_builder()
            .id(self.id.into())
            .minimal_slot(self.minimal_slot.pack())
            .maximal_slot(self.maximal_slot.pack())
            .tip_valid_header_root(self.tip_valid_header_root.pack())
            .headers_mmr_root(self.headers_mmr_root.pack())
            .build()
    }
}

impl Pack<packed::ClientTypeArgs> for core::ClientTypeArgs {
    fn pack(&self) -> packed::ClientTypeArgs {
        packed::ClientTypeArgs::new_builder()
            .type_id(self.type_id.pack())
            .cells_count(self.cells_count.into())
            .build()
    }
}
