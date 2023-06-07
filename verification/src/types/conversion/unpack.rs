use bls::{PUBLIC_KEY_BYTES_LEN, SIGNATURE_BYTES_LEN};
use molecule::prelude::*;

use crate::types::{core, packed, prelude::Unpack};

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

impl<'r> Unpack<core::BlsPubkey> for packed::BlsPubkeyReader<'r> {
    fn unpack(&self) -> core::BlsPubkey {
        assert_eq!(self.as_slice().len(), PUBLIC_KEY_BYTES_LEN);
        let mut b = [0u8; PUBLIC_KEY_BYTES_LEN];
        b.copy_from_slice(self.as_slice());
        b.into()
    }
}
impl_conversion_for_entity_unpack!(BlsPubkey);

impl<'r> Unpack<core::BlsSignature> for packed::BlsSignatureReader<'r> {
    fn unpack(&self) -> core::BlsSignature {
        assert_eq!(self.as_slice().len(), SIGNATURE_BYTES_LEN);
        let mut b = [0u8; SIGNATURE_BYTES_LEN];
        b.copy_from_slice(self.as_slice());
        b.into()
    }
}
impl_conversion_for_entity_unpack!(BlsSignature);

impl<'r> Unpack<core::BlsPubkeyVec> for packed::BlsPubkeyVecReader<'r> {
    fn unpack(&self) -> core::BlsPubkeyVec {
        self.iter().map(|v| v.unpack()).collect()
    }
}
impl_conversion_for_entity_unpack!(BlsPubkeyVec);

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

impl<'r> Unpack<core::HeaderVec> for packed::HeaderVecReader<'r> {
    fn unpack(&self) -> core::HeaderVec {
        self.iter().map(|v| v.unpack()).collect()
    }
}
impl_conversion_for_entity_unpack!(HeaderVec);

impl<'r> Unpack<core::SyncCommitteeBits> for packed::SyncCommitteeBitsReader<'r> {
    fn unpack(&self) -> core::SyncCommitteeBits {
        core::SyncCommitteeBits::from_slice(self.as_slice())
    }
}
impl_conversion_for_entity_unpack!(SyncCommitteeBits);

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

impl<'r> Unpack<core::ClientBootstrap> for packed::ClientBootstrapReader<'r> {
    fn unpack(&self) -> core::ClientBootstrap {
        core::ClientBootstrap {
            header: self.header().unpack(),
            current_sync_committee_branch: self.current_sync_committee_branch().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(ClientBootstrap);

impl<'r> Unpack<core::ClientUpdate> for packed::ClientUpdateReader<'r> {
    fn unpack(&self) -> core::ClientUpdate {
        core::ClientUpdate {
            attested_header: self.attested_header().unpack(),
            finality_branch: self.finality_branch().unpack(),
            sync_aggregate: self.sync_aggregate().unpack(),
            signature_slot: self.signature_slot().unpack(),
            new_headers_mmr_proof: self.new_headers_mmr_proof().unpack(),
            headers: self.headers().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(ClientUpdate);

impl<'r> Unpack<core::SyncCommitteeUpdate> for packed::SyncCommitteeUpdateReader<'r> {
    fn unpack(&self) -> core::SyncCommitteeUpdate {
        core::SyncCommitteeUpdate {
            attested_header: self.attested_header().unpack(),
            next_sync_committee_branch: self.next_sync_committee_branch().unpack(),
            sync_aggregate: self.sync_aggregate().unpack(),
            signature_slot: self.signature_slot().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(SyncCommitteeUpdate);

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

impl<'r> Unpack<core::ClientInfo> for packed::ClientInfoReader<'r> {
    fn unpack(&self) -> core::ClientInfo {
        core::ClientInfo {
            last_client_id: self.last_client_id().into(),
            minimal_headers_count: self.minimal_headers_count().into(),
            genesis_validators_root: self.genesis_validators_root().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(ClientInfo);

impl<'r> Unpack<core::Client> for packed::ClientReader<'r> {
    fn unpack(&self) -> core::Client {
        core::Client {
            id: self.id().into(),
            minimal_slot: self.minimal_slot().unpack(),
            maximal_slot: self.maximal_slot().unpack(),
            tip_header_root: self.tip_header_root().unpack(),
            headers_mmr_root: self.headers_mmr_root().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(Client);

impl<'r> Unpack<core::ClientSyncCommittee> for packed::ClientSyncCommitteeReader<'r> {
    fn unpack(&self) -> core::ClientSyncCommittee {
        core::ClientSyncCommittee {
            period: self.period().unpack(),
            data: self.data().unpack(),
        }
    }
}
impl_conversion_for_entity_unpack!(ClientSyncCommittee);

impl<'r> Unpack<core::ClientTypeArgs> for packed::ClientTypeArgsReader<'r> {
    fn unpack(&self) -> core::ClientTypeArgs {
        core::ClientTypeArgs {
            type_id: self.type_id().unpack(),
            clients_count: self.clients_count().into(),
        }
    }
}
impl_conversion_for_entity_unpack!(ClientTypeArgs);
