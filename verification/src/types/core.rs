#[cfg(feature = "std")]
use alloc::fmt;

use molecule::prelude::*;
use ssz_derive::Encode;
use tree_hash::Hash256;
use tree_hash_derive::TreeHash;

pub type Uint64 = u64;
pub type Hash = Hash256;
pub type Bytes = Vec<u8>;

pub type Bytes64 = [u8; 64];
pub type BlsPubkey = [u8; 32];
pub type BlsSignature = [u8; 96];

pub type BlsPubkeyArray = [BlsPubkey; 512];

pub type SszProof = Vec<Hash>;
pub type MptProof = Vec<Bytes>;

#[derive(Clone)]
pub struct HeaderDigest {
    pub children_hash: Hash,
}

pub type MmrProof = Vec<HeaderDigest>;

#[derive(Clone, Encode, TreeHash)]
pub struct Header {
    pub slot: Uint64,
    pub proposer_index: Uint64,
    pub parent_root: Hash,
    pub state_root: Hash,
    pub body_root: Hash,
}

#[derive(Clone)]
pub struct SyncAggregate {
    pub sync_committee_bits: Bytes64,
    pub sync_committee_signature: BlsSignature,
}

#[derive(Clone)]
pub struct FinalityUpdate {
    pub attested_header: Header,
    pub finalized_header: Header,
    pub finality_branch: SszProof,
}

#[derive(Clone)]
pub struct SyncCommittee {
    pub period: Uint64,
    pub pubkeys: BlsPubkeyArray,
    pub aggregate_pubkey: BlsPubkey,
}

pub type HeaderVec = Vec<Header>;
pub type FinalityUpdateVec = Vec<FinalityUpdate>;

#[derive(Clone)]
pub struct Client {
    pub minimal_slot: Uint64,
    pub maximal_slot: Uint64,
    pub tip_valid_header_root: Hash,
    pub headers_mmr_root: HeaderDigest,
}

#[derive(Clone)]
pub struct ProofUpdate {
    pub new_headers_mmr_root: HeaderDigest,
    pub next_committee_ssz_proof: SszProof,
    pub new_headers_mmr_proof: MmrProof,
    pub updates: FinalityUpdateVec,
}

#[derive(Clone)]
pub struct TransactionProof {
    pub header: Header,
    pub transaction_index: Uint64,
    pub receipts_root: Hash,
    pub header_mmr_proof: MmrProof,
    pub transaction_ssz_proof: SszProof,
    pub receipt_mpt_proof: MptProof,
    pub receipts_root_ssz_proof: SszProof,
}

#[derive(Clone)]
pub struct TransactionPayload {
    pub transaction: Bytes,
    pub receipt: Bytes,
}

#[cfg(feature = "std")]
impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "{{ slot: {}, empty: true }}", self.slot)
        } else if f.alternate() {
            write!(
                f,
                "{{ slot: {}, parent: {:#x}, state: {:#x}, body: {:#x} }}",
                self.slot, self.parent_root, self.state_root, self.body_root
            )
        } else {
            write!(
                f,
                "{{ slot: {}, parent: {:#x} }}",
                self.slot, self.parent_root
            )
        }
    }
}

#[cfg(feature = "std")]
impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ slots: [{}, {}], tip: {:#x} }}",
            self.minimal_slot, self.maximal_slot, self.tip_valid_header_root
        )
    }
}
