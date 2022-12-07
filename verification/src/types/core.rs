use molecule::prelude::*;
use tree_hash::Hash256;

pub type Uint64 = u64;
pub type Hash = Hash256;
pub type Bytes = Vec<u8>;

pub type Bytes64 = [u8; 64];
pub type BlsPubkey = [u8; 32];
pub type BlsSignature = [u8; 96];

pub type BlsPubkeyArray = [BlsPubkey; 512];

pub type SszProof = Vec<Hash>;
pub type MptProof = Vec<Bytes>;

pub struct HeaderDigest {
    pub start_slot: Uint64,
    pub end_slot: Uint64,
    pub mmr_hash: Hash,
}

pub type MmrProof = Vec<HeaderDigest>;

pub struct Eth2Header {
    pub slot: Uint64,
    pub body_root: Hash,
}

pub struct SyncAggregate {
    pub sync_committee_bits: Bytes64,
    pub sync_committee_signature: BlsSignature,
}

pub struct SyncCommittee {
    pub pubkeys: BlsPubkeyArray,
    pub aggregate_pubkey: BlsPubkey,
}

pub type Eth2HeaderVec = Vec<Eth2Header>;
pub type Eth2UpdateVec = Vec<Bytes>;

pub struct Client {
    pub minimal_slot: Uint64,
    pub maximal_slot: Uint64,
    pub headers_mmr_root: HeaderDigest,
}

pub struct HeadersUpdate {
    pub tip_header: Eth2Header,
    pub tip_header_mmr_proof: MmrProof,
    pub headers: Eth2HeaderVec,
    pub updates: Eth2UpdateVec,
    pub new_headers_mmr_root: HeaderDigest,
    pub new_headers_mmr_proof: MmrProof,
}

pub struct TransactionProof {
    pub header: Eth2Header,
    pub transaction_index: Uint64,
    pub receipts_root: Hash,
    pub header_mmr_proof: MmrProof,
    pub transaction_ssz_proof: SszProof,
    pub receipt_mpt_proof: MptProof,
    pub receipts_root_ssz_proof: SszProof,
}

pub struct TransactionPayload {
    pub transaction: Bytes,
    pub receipt: Bytes,
}
