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
            start_slot: self.start_slot().unpack(),
            end_slot: self.end_slot().unpack(),
            mmr_hash: self.mmr_hash().unpack(),
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

impl<'r> Unpack<core::Eth2Header> for packed::Eth2HeaderReader<'r> {
    fn unpack(&self) -> core::Eth2Header {
        core::Eth2Header {
            slot: self.slot().unpack(),
            body_root: self.body_root().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(Eth2Header);

impl<'r> Unpack<core::SyncAggregate> for packed::SyncAggregateReader<'r> {
    fn unpack(&self) -> core::SyncAggregate {
        core::SyncAggregate {
            sync_committee_bits: self.sync_committee_bits().unpack(),
            sync_committee_signature: self.sync_committee_signature().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(SyncAggregate);

impl<'r> Unpack<core::SyncCommittee> for packed::SyncCommitteeReader<'r> {
    fn unpack(&self) -> core::SyncCommittee {
        core::SyncCommittee {
            pubkeys: self.pubkeys().unpack(),
            aggregate_pubkey: self.aggregate_pubkey().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(SyncCommittee);

impl<'r> Unpack<core::Eth2HeaderVec> for packed::Eth2HeaderVecReader<'r> {
    fn unpack(&self) -> core::Eth2HeaderVec {
        self.iter().map(|v| v.unpack()).collect()
    }
}
impl_conversion_for_entity_unpack!(Eth2HeaderVec);

impl<'r> Unpack<core::Eth2UpdateVec> for packed::Eth2UpdateVecReader<'r> {
    fn unpack(&self) -> core::Eth2UpdateVec {
        self.iter().map(|v| v.unpack()).collect()
    }
}
impl_conversion_for_entity_unpack!(Eth2UpdateVec);

impl<'r> Unpack<core::Client> for packed::ClientReader<'r> {
    fn unpack(&self) -> core::Client {
        core::Client {
            minimal_slot: self.minimal_slot().unpack(),
            maximal_slot: self.maximal_slot().unpack(),
            headers_mmr_root: self.headers_mmr_root().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(Client);

impl<'r> Unpack<core::HeadersUpdate> for packed::HeadersUpdateReader<'r> {
    fn unpack(&self) -> core::HeadersUpdate {
        core::HeadersUpdate {
            tip_header: self.tip_header().unpack(),
            tip_header_mmr_proof: self.tip_header_mmr_proof().unpack(),
            headers: self.headers().unpack(),
            updates: self.updates().unpack(),
            new_headers_mmr_root: self.new_headers_mmr_root().unpack(),
            new_headers_mmr_proof: self.new_headers_mmr_proof().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(HeadersUpdate);

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
