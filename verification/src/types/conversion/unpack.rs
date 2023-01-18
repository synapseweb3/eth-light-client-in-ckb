use molecule::prelude::*;

use crate::types::{core, packed, prelude::*};

macro_rules! impl_conversion_for_entity_unpack {
    ($name:ident) => {
        impl Unpack<core::$name> for packed::$name {
            fn unpack(&self) -> core::$name {
                self.as_reader().unpack()
            }
        }
    };
}

impl<'r> Unpack<core::Uint64> for packed::Uint64Reader<'r> {
    fn unpack(&self) -> core::Uint64 {
        let mut b = [0u8; 8];
        b.copy_from_slice(self.as_slice());
        core::Uint64::from_le_bytes(b)
    }
}
impl_conversion_for_entity_unpack!(Uint64);

impl<'r> Unpack<core::Hash> for packed::HashReader<'r> {
    fn unpack(&self) -> core::Hash {
        core::Hash::from_slice(self.as_slice())
    }
}
impl_conversion_for_entity_unpack!(Hash);

impl<'r> Unpack<core::Bytes> for packed::BytesReader<'r> {
    fn unpack(&self) -> core::Bytes {
        self.raw_data().to_owned()
    }
}
impl_conversion_for_entity_unpack!(Bytes);

impl<'r> Unpack<core::Bytes64> for packed::Bytes64Reader<'r> {
    fn unpack(&self) -> core::Bytes64 {
        let mut array = [0u8; 64];
        array.copy_from_slice(self.as_slice());
        array
    }
}
impl_conversion_for_entity_unpack!(Bytes64);

impl<'r> Unpack<core::BlsPubkey> for packed::BlsPubkeyReader<'r> {
    fn unpack(&self) -> core::BlsPubkey {
        let mut array = [0u8; 32];
        array.copy_from_slice(self.as_slice());
        array
    }
}
impl_conversion_for_entity_unpack!(BlsPubkey);

impl<'r> Unpack<core::BlsSignature> for packed::BlsSignatureReader<'r> {
    fn unpack(&self) -> core::BlsSignature {
        let mut array = [0u8; 96];
        array.copy_from_slice(self.as_slice());
        array
    }
}
impl_conversion_for_entity_unpack!(BlsSignature);

impl<'r> Unpack<core::BlsPubkeyArray> for packed::BlsPubkeyArrayReader<'r> {
    fn unpack(&self) -> core::BlsPubkeyArray {
        let mut array = [[0u8; 32]; 512];
        for (i, item) in array.iter_mut().enumerate() {
            let start = 32 * i;
            let end = start + 32;
            item.copy_from_slice(&self.as_slice()[start..end]);
        }
        array
    }
}
impl_conversion_for_entity_unpack!(BlsPubkeyArray);

impl<'r> Unpack<core::SszProof> for packed::SszProofReader<'r> {
    fn unpack(&self) -> core::SszProof {
        self.iter().map(|v| v.unpack()).collect()
    }
}
impl_conversion_for_entity_unpack!(SszProof);

impl<'r> Unpack<core::MptProof> for packed::MptProofReader<'r> {
    fn unpack(&self) -> core::MptProof {
        self.iter().map(|v| v.unpack()).collect()
    }
}
impl_conversion_for_entity_unpack!(MptProof);

impl<'r> Unpack<core::HeaderDigest> for packed::HeaderDigestReader<'r> {
    fn unpack(&self) -> core::HeaderDigest {
        core::HeaderDigest {
            children_hash: self.children_hash().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(HeaderDigest);

impl<'r> Unpack<core::MmrProof> for packed::MmrProofReader<'r> {
    fn unpack(&self) -> core::MmrProof {
        self.iter().map(|v| v.unpack()).collect()
    }
}
impl_conversion_for_entity_unpack!(MmrProof);

impl<'r> Unpack<core::Header> for packed::HeaderReader<'r> {
    fn unpack(&self) -> core::Header {
        core::Header {
            slot: self.slot().unpack(),
            proposer_index: self.proposer_index().unpack(),
            parent_root: self.parent_root().unpack(),
            state_root: self.state_root().unpack(),
            body_root: self.body_root().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(Header);

impl<'r> Unpack<core::SyncAggregate> for packed::SyncAggregateReader<'r> {
    fn unpack(&self) -> core::SyncAggregate {
        core::SyncAggregate {
            sync_committee_bits: self.sync_committee_bits().unpack(),
            sync_committee_signature: self.sync_committee_signature().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(SyncAggregate);

impl<'r> Unpack<core::FinalityUpdate> for packed::FinalityUpdateReader<'r> {
    fn unpack(&self) -> core::FinalityUpdate {
        core::FinalityUpdate {
            attested_header: self.attested_header().unpack(),
            finalized_header: self.finalized_header().unpack(),
            finality_branch: self.finality_branch().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(FinalityUpdate);

impl<'r> Unpack<core::SyncCommittee> for packed::SyncCommitteeReader<'r> {
    fn unpack(&self) -> core::SyncCommittee {
        core::SyncCommittee {
            period: self.period().unpack(),
            pubkeys: self.pubkeys().unpack(),
            aggregate_pubkey: self.aggregate_pubkey().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(SyncCommittee);

impl<'r> Unpack<core::HeaderVec> for packed::HeaderVecReader<'r> {
    fn unpack(&self) -> core::HeaderVec {
        self.iter().map(|v| v.unpack()).collect()
    }
}
impl_conversion_for_entity_unpack!(HeaderVec);

impl<'r> Unpack<core::FinalityUpdateVec> for packed::FinalityUpdateVecReader<'r> {
    fn unpack(&self) -> core::FinalityUpdateVec {
        self.iter().map(|v| v.unpack()).collect()
    }
}
impl_conversion_for_entity_unpack!(FinalityUpdateVec);

impl<'r> Unpack<core::Client> for packed::ClientReader<'r> {
    fn unpack(&self) -> core::Client {
        core::Client {
            minimal_slot: self.minimal_slot().unpack(),
            maximal_slot: self.maximal_slot().unpack(),
            tip_header_root: self.tip_header_root().unpack(),
            headers_mmr_root: self.headers_mmr_root().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(Client);

impl<'r> Unpack<core::ProofUpdate> for packed::ProofUpdateReader<'r> {
    fn unpack(&self) -> core::ProofUpdate {
        core::ProofUpdate {
            new_headers_mmr_root: self.new_headers_mmr_root().unpack(),
            next_committee_ssz_proof: self.next_committee_ssz_proof().unpack(),
            new_headers_mmr_proof: self.new_headers_mmr_proof().unpack(),
            updates: self.updates().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(ProofUpdate);

impl<'r> Unpack<core::TransactionProof> for packed::TransactionProofReader<'r> {
    fn unpack(&self) -> core::TransactionProof {
        core::TransactionProof {
            header: self.header().unpack(),
            transaction_index: self.transaction_index().unpack(),
            receipts_root: self.receipts_root().unpack(),
            header_mmr_proof: self.header_mmr_proof().unpack(),
            transaction_ssz_proof: self.transaction_ssz_proof().unpack(),
            receipt_mpt_proof: self.receipt_mpt_proof().unpack(),
            receipts_root_ssz_proof: self.receipts_root_ssz_proof().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(TransactionProof);

impl<'r> Unpack<core::TransactionPayload> for packed::TransactionPayloadReader<'r> {
    fn unpack(&self) -> core::TransactionPayload {
        core::TransactionPayload {
            transaction: self.transaction().unpack(),
            receipt: self.receipt().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(TransactionPayload);
