use alloc::format;
use core::cmp::PartialEq;

use ckb_mmr::{Error as MMRError, Merge, MerkleProof, Result as MMRResult, MMR};
use eth2_hashing::{hash32_concat, hash_fixed, HASH_LEN};
use tree_hash::Hash256;

use crate::types::{packed, prelude::*};

/// A struct to implement MMR `Merge` trait
pub struct MergeHeaderDigest;
/// MMR root
pub type ClientRootMMR<S> = MMR<packed::HeaderDigest, MergeHeaderDigest, S>;
/// MMR proof
pub type MMRProof = MerkleProof<packed::HeaderDigest, MergeHeaderDigest>;

impl<'r> packed::Eth2HeaderReader<'r> {
    /// Get the MMR header digest from the header
    pub fn digest(&self) -> packed::HeaderDigest {
        packed::HeaderDigest::new_builder()
            .start_slot(self.slot().to_entity())
            .end_slot(self.slot().to_entity())
            .mmr_hash(self.body_root().to_entity())
            .build()
    }
}

impl packed::Eth2Header {
    pub fn digest(&self) -> packed::HeaderDigest {
        self.as_reader().digest()
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
        let mmr_hash = hash32_concat(&lhs.calc_mmr_hash(), &rhs.calc_mmr_hash());

        let lhs_end_slot = lhs.end_slot().unpack();
        let rhs_start_slot = rhs.start_slot().unpack();
        if lhs_end_slot + 1 != rhs_start_slot {
            let errmsg = format!(
                "failed since the headers isn't continuous ([-,{}], [{},-])",
                lhs_end_slot, rhs_start_slot
            );
            return Err(MMRError::MergeError(errmsg));
        }

        Ok(Self::Item::new_builder()
            .start_slot(lhs.start_slot())
            .end_slot(rhs.end_slot())
            .mmr_hash(Hash256::from(mmr_hash).pack())
            .build())
    }

    fn merge_peaks(lhs: &Self::Item, rhs: &Self::Item) -> MMRResult<Self::Item> {
        Self::merge(rhs, lhs)
    }
}
