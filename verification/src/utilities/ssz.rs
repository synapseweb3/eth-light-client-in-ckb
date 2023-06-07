//! The utilities for [SimpleSerialize (SSZ)].
//!
//! [SimpleSerialize (SSZ)]: https://github.com/ethereum/consensus-specs/blob/v1.1.0/ssz/simple-serialize.md

use core::mem;

use eth2_hashing::hash32_concat;
use tree_hash::{Hash256, BYTES_PER_CHUNK};

pub const fn ceil_depth(num: usize) -> u32 {
    let next_power_of_two = num.next_power_of_two();
    usize::BITS - next_power_of_two.leading_zeros() - 1
}

pub const fn floor_depth(num: usize) -> u32 {
    if num == 0 {
        0
    } else {
        let next_power_of_two = num.next_power_of_two();
        usize::BITS
            - next_power_of_two.leading_zeros()
            - if num == next_power_of_two { 1 } else { 2 }
    }
}

/// Merkleizes a length.
///
/// See [Merkleization] for more details.
///
/// [Merkleization]: https://github.com/ethereum/consensus-specs/blob/v1.1.0/ssz/simple-serialize.md#merkleization
pub fn length_hash(length: usize) -> Hash256 {
    let usize_len = mem::size_of::<usize>();
    let mut length_bytes = [0; BYTES_PER_CHUNK];
    length_bytes[0..usize_len].copy_from_slice(&length.to_le_bytes());
    Hash256::from(length_bytes)
}

/// Verifies a SimpleSerialize (SSZ) Merkle Proof.
///
/// See `verify_merkle_proof` in [Merkle multiproofs] for more details.
///
/// [Merkle multiproofs]: https://github.com/ethereum/consensus-specs/blob/v1.1.0/ssz/merkle-proofs.md#merkle-multiproofs
pub fn verify_merkle_proof(
    root: &Hash256,
    leaf: &Hash256,
    proof: &[Hash256],
    index: usize,
) -> bool {
    assert_eq!(proof.len(), get_generalized_index_length(index));
    calculate_merkle_root(leaf, proof, index) == *root
}

/// Verifies a SimpleSerialize (SSZ) Merkle Proof.
///
/// See `verify_merkle_proof` in [Merkle multiproofs] for more details.
///
/// [Merkle multiproofs]: https://github.com/ethereum/consensus-specs/blob/v1.1.0/ssz/merkle-proofs.md#merkle-multiproofs
pub fn calculate_merkle_root(leaf: &Hash256, proof: &[Hash256], index: usize) -> Hash256 {
    let mut hash = leaf.to_fixed_bytes();
    for (i, item) in proof.iter().enumerate() {
        if get_generalized_index_bit(index, i) {
            hash = hash32_concat(item.as_ref(), &hash);
        } else {
            hash = hash32_concat(&hash, item.as_ref());
        }
    }
    Hash256::from(hash)
}

/// Returns the given bit of a generalized index.
///
/// See [`get_generalized_index_bit`].
///
/// [`get_generalized_index_bit`]: https://github.com/ethereum/consensus-specs/blob/v1.1.0/ssz/merkle-proofs.md#get_generalized_index_bit
const fn get_generalized_index_bit(index: usize, pos: usize) -> bool {
    let index = index as u64;
    (index & (1u64 << pos)) > 0
}

/// Returns the length of a path represented by a generalized index.
///
/// See [`get_generalized_index_length`].
///
/// [`get_generalized_index_length`]: https://github.com/ethereum/consensus-specs/blob/v1.1.0/ssz/merkle-proofs.md#get_generalized_index_length
const fn get_generalized_index_length(index: usize) -> usize {
    floor_depth(index) as usize
}

/// Checks if `leaf` at `index` verifies against the Merkle `root` and `branch`.
///
/// See [`is_valid_merkle_branch`].
///
/// [`is_valid_merkle_branch`]: https://github.com/ethereum/consensus-specs/blob/v1.0.0/specs/phase0/beacon-chain.md#is_valid_merkle_branch
pub fn is_valid_merkle_branch(
    leaf: &Hash256,
    branch: &[Hash256],
    depth: usize,
    index: usize,
    root: &Hash256,
) -> bool {
    let mut value = leaf.to_fixed_bytes();
    if branch.len() < depth {
        return false;
    }
    let mut tmp = 1; // 2**i, i = 0, 1, ..., depth-1
    for node in branch.iter().take(depth) {
        if index / tmp % 2 > 0 {
            value = hash32_concat(node.as_ref(), &value);
        } else {
            value = hash32_concat(&value, node.as_ref());
        }
        tmp *= 2;
    }
    Hash256::from(value) == *root
}
