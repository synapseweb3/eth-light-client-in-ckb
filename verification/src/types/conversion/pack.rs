use molecule::prelude::*;

use crate::{
    constants::consensus_specs,
    types::{core, packed, prelude::*},
};

impl Pack<packed::Uint64> for core::Uint64 {
    fn pack(&self) -> packed::Uint64 {
        packed::Uint64::new_unchecked(molecule::bytes::Bytes::from(self.to_le_bytes().to_vec()))
    }
}

impl Pack<packed::Hash> for core::Hash {
    fn pack(&self) -> packed::Hash {
        packed::Hash::new_unchecked(molecule::bytes::Bytes::from(self.as_bytes()))
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

impl Pack<packed::Bytes64> for core::Bytes64 {
    fn pack(&self) -> packed::Bytes64 {
        packed::Bytes64::new_unchecked(molecule::bytes::Bytes::from(self.to_vec()))
    }
}

impl Pack<packed::BlsPubkey> for core::BlsPubkey {
    fn pack(&self) -> packed::BlsPubkey {
        packed::BlsPubkey::new_unchecked(molecule::bytes::Bytes::from(self.to_vec()))
    }
}

impl Pack<packed::BlsSignature> for core::BlsSignature {
    fn pack(&self) -> packed::BlsSignature {
        packed::BlsSignature::new_unchecked(molecule::bytes::Bytes::from(self.to_vec()))
    }
}

impl Pack<packed::BlsPubkeyArray> for core::BlsPubkeyArray {
    fn pack(&self) -> packed::BlsPubkeyArray {
        let mut vec = Vec::with_capacity(32 * consensus_specs::SYNC_COMMITTEE_SIZE);
        for pubkey in self {
            vec.extend_from_slice(pubkey.as_ref());
        }
        packed::BlsPubkeyArray::new_unchecked(molecule::bytes::Bytes::from(vec))
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
            .start_slot(self.start_slot.pack())
            .end_slot(self.end_slot.pack())
            .mmr_hash(self.mmr_hash.pack())
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

impl Pack<packed::Eth2Header> for core::Eth2Header {
    fn pack(&self) -> packed::Eth2Header {
        packed::Eth2Header::new_builder()
            .slot(self.slot.pack())
            .body_root(self.body_root.pack())
            .build()
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

impl Pack<packed::Eth2HeaderVec> for core::Eth2HeaderVec {
    fn pack(&self) -> packed::Eth2HeaderVec {
        packed::Eth2HeaderVec::new_builder()
            .set(self.iter().map(|v| v.pack()).collect())
            .build()
    }
}

impl Pack<packed::Eth2UpdateVec> for core::Eth2UpdateVec {
    fn pack(&self) -> packed::Eth2UpdateVec {
        packed::Eth2UpdateVec::new_builder()
            .set(self.iter().map(|v| v.pack()).collect())
            .build()
    }
}

impl Pack<packed::Client> for core::Client {
    fn pack(&self) -> packed::Client {
        packed::Client::new_builder()
            .minimal_slot(self.minimal_slot.pack())
            .maximal_slot(self.maximal_slot.pack())
            .headers_mmr_root(self.headers_mmr_root.pack())
            .build()
    }
}

impl Pack<packed::HeadersUpdate> for core::HeadersUpdate {
    fn pack(&self) -> packed::HeadersUpdate {
        packed::HeadersUpdate::new_builder()
            .tip_header(self.tip_header.pack())
            .tip_header_mmr_proof(self.tip_header_mmr_proof.pack())
            .headers(self.headers.pack())
            .updates(self.updates.pack())
            .new_headers_mmr_root(self.new_headers_mmr_root.pack())
            .new_headers_mmr_proof(self.new_headers_mmr_proof.pack())
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
