#[cfg(feature = "std")]
use alloc::fmt;

use molecule::prelude::*;
use ssz_derive::Encode;
use tree_hash::Hash256;
use tree_hash_derive::TreeHash;

pub type Uint64 = u64;
pub type Hash = Hash256;
pub type Bytes = Vec<u8>;

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

pub type HeaderVec = Vec<Header>;

#[derive(Clone)]
pub struct ProofUpdate {
    pub new_headers_mmr_root: HeaderDigest,
    pub new_headers_mmr_proof: MmrProof,
    pub updates: HeaderVec,
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

#[derive(Clone)]
pub struct ClientInfo {
    pub last_id: u8,
    pub minimal_updates_count: u8,
}

#[derive(Clone)]
pub struct Client {
    pub id: u8,
    pub minimal_slot: Uint64,
    pub maximal_slot: Uint64,
    pub tip_valid_header_root: Hash,
    pub headers_mmr_root: HeaderDigest,
}

#[derive(Clone)]
pub struct ClientTypeArgs {
    pub type_id: Hash,
    pub cells_count: u8,
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
impl fmt::Display for ClientInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ last_id: {}, minimal_updates_count: {} }}",
            self.last_id, self.minimal_updates_count
        )
    }
}

#[cfg(feature = "std")]
impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ id: {}, slots: [{}, {}], tip: {:#x} }}",
            self.id, self.minimal_slot, self.maximal_slot, self.tip_valid_header_root
        )
    }
}
