use core::mem;

use eth2_hashing::hash32_concat;
use tree_hash::{Hash256, BYTES_PER_CHUNK};

pub const fn ceil_depth(num: usize) -> u32 {
    let next_power_of_two = num.next_power_of_two();
    usize::BITS - next_power_of_two.leading_zeros() - 1
}

pub(crate) const fn floor_depth(num: usize) -> u32 {
    if num == 0 {
        0
    } else {
        let next_power_of_two = num.next_power_of_two();
        usize::BITS
            - next_power_of_two.leading_zeros()
            - if num == next_power_of_two { 1 } else { 2 }
    }
}

pub fn length_hash(length: usize) -> Hash256 {
    let usize_len = mem::size_of::<usize>();
    let mut length_bytes = [0; BYTES_PER_CHUNK];
    length_bytes[0..usize_len].copy_from_slice(&length.to_le_bytes());
    Hash256::from(length_bytes)
}

pub fn verify_merkle_proof(root: Hash256, leaf: Hash256, proof: &[Hash256], index: usize) -> bool {
    if proof.len() != get_generalized_index_length(index) {
        return false;
    }
    calculate_merkle_root(leaf, proof, index) == root
}

fn calculate_merkle_root(leaf: Hash256, proof: &[Hash256], index: usize) -> Hash256 {
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

const fn get_generalized_index_bit(index: usize, pos: usize) -> bool {
    let index = index as u64;
    (index & (1u64 << pos)) > 0
}

const fn get_generalized_index_length(index: usize) -> usize {
    floor_depth(index) as usize
}
