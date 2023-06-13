//! The utilities for [Merkle Mountain Ranges (MMR)].
//!
//! [Merkle Mountain Ranges (MMR)]: https://github.com/opentimestamps/opentimestamps-server/blob/master/doc/merkle-mountain-range.md

use ::core::cmp::PartialEq;
#[cfg(feature = "std")]
use alloc::fmt;

use ckb_mmr::{Merge, MerkleProof, Result as MMRResult, MMR};
use eth2_hashing::{hash32_concat, hash_fixed, HASH_LEN};
use tree_hash::{Hash256, TreeHash as _};

use crate::types::{core, packed, prelude::*};

pub use ckb_mmr as lib;

/// A struct to implement MMR `Merge` trait.
pub struct MergeHeaderDigest;
/// MMR root.
pub type ClientRootMMR<S> = MMR<packed::HeaderDigest, MergeHeaderDigest, S>;
/// MMR proof.
pub type MMRProof = MerkleProof<packed::HeaderDigest, MergeHeaderDigest>;

/// Header with the cached root.
#[derive(Clone)]
pub struct HeaderWithCache {
    pub inner: core::Header,
    pub root: Hash256,
}

#[cfg(feature = "std")]
impl fmt::Display for HeaderWithCache {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(
                f,
                "{{ slot: {}, root: {:#x}, empty: true }}",
                self.inner.slot, self.root
            )
        } else if f.alternate() {
            write!(
                f,
                "{{ slot: {}, root: {:#x}, parent: {:#x}, state: {:#x}, body: {:#x} }}",
                self.inner.slot,
                self.root,
                self.inner.parent_root,
                self.inner.state_root,
                self.inner.body_root
            )
        } else {
            write!(
                f,
                "{{ slot: {}, root: {:#x}, parent: {:#x} }}",
                self.inner.slot, self.root, self.inner.parent_root
            )
        }
    }
}

impl core::Header {
    /// Calculates the root of a header and caches the root.
    pub fn calc_cache(self) -> HeaderWithCache {
        let root = self.tree_hash_root();
        HeaderWithCache { inner: self, root }
    }
}

impl HeaderWithCache {
    /// Builds a `HeaderDigest`.
    pub fn digest(&self) -> core::HeaderDigest {
        core::HeaderDigest {
            children_hash: self.root,
        }
    }

    /// Builds a packed `HeaderDigest` as MMR node.
    pub fn packed_digest(&self) -> packed::HeaderDigest {
        packed::HeaderDigest::new_builder()
            .children_hash(self.root.pack())
            .build()
    }

    /// Checks if a header is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<'r> packed::HeaderDigestReader<'r> {
    /// Calculates the MMR hash root for the current MMR node.
    pub fn calc_mmr_hash(&self) -> [u8; HASH_LEN] {
        hash_fixed(self.as_slice())
    }
}

impl packed::HeaderDigest {
    /// Calculates the MMR hash root for the current MMR node.
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
