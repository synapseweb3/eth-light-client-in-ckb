use core::cmp::PartialEq;

use ckb_mmr::{Merge, MerkleProof, Result as MMRResult, MMR};
use eth2_hashing::{hash32_concat, hash_fixed, HASH_LEN};
#[cfg(feature = "std")]
use eth2_types::{BeaconBlockHeader, Slot};
use tree_hash::{Hash256, TreeHash as _};

use crate::types::{core::Header, packed, prelude::*};

pub use ckb_mmr as lib;

/// A struct to implement MMR `Merge` trait
pub struct MergeHeaderDigest;
/// MMR root
pub type ClientRootMMR<S> = MMR<packed::HeaderDigest, MergeHeaderDigest, S>;
/// MMR proof
pub type MMRProof = MerkleProof<packed::HeaderDigest, MergeHeaderDigest>;

// Header with the cached root.
pub struct HeaderWithCache {
    pub inner: Header,
    pub root: Hash256,
}

impl Header {
    pub fn calc_cache(self) -> HeaderWithCache {
        let root = self.tree_hash_root();
        HeaderWithCache { inner: self, root }
    }
}

impl HeaderWithCache {
    pub fn digest(&self) -> packed::HeaderDigest {
        packed::HeaderDigest::new_builder()
            .children_hash(self.root.pack())
            .build()
    }
}

impl<'r> packed::HeaderReader<'r> {
    #[cfg(feature = "std")]
    pub fn to_ssz_header(&self) -> BeaconBlockHeader {
        BeaconBlockHeader {
            slot: Slot::new(self.slot().unpack()),
            proposer_index: self.proposer_index().unpack(),
            parent_root: self.parent_root().unpack(),
            state_root: self.state_root().unpack(),
            body_root: self.body_root().unpack(),
        }
    }
}

impl packed::Header {
    #[cfg(feature = "std")]
    pub fn from_ssz_header(header: &BeaconBlockHeader) -> Self {
        let slot: u64 = header.slot.into();
        packed::Header::new_builder()
            .slot(slot.pack())
            .proposer_index(header.proposer_index.pack())
            .parent_root(header.parent_root.pack())
            .state_root(header.state_root.pack())
            .body_root(header.body_root.pack())
            .build()
    }
}

impl<'r> packed::HeaderDigestReader<'r> {
    pub fn calc_mmr_hash(&self) -> [u8; HASH_LEN] {
        hash_fixed(self.as_slice())
    }
}

impl packed::HeaderDigest {
    pub fn calc_mmr_hash(&self) -> [u8; HASH_LEN] {
        self.as_reader().calc_mmr_hash()
    }
}

impl PartialEq for packed::HeaderDigest {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Merge for MergeHeaderDigest {
    type Item = packed::HeaderDigest;

    fn merge(lhs: &Self::Item, rhs: &Self::Item) -> MMRResult<Self::Item> {
        let children_hash = hash32_concat(&lhs.calc_mmr_hash(), &rhs.calc_mmr_hash());
        Ok(Self::Item::new_builder()
            .children_hash(Hash256::from(children_hash).pack())
            .build())
    }

    fn merge_peaks(lhs: &Self::Item, rhs: &Self::Item) -> MMRResult<Self::Item> {
        Self::merge(rhs, lhs)
    }
}
