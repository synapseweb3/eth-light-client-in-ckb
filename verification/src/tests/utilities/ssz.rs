use alloc::{vec, vec::Vec};

// TODO fix no-std
use core as std;

use merkle_proof::MerkleTree;
use ssz_derive::{Decode, Encode};
use ssz_types::{typenum, VariableList};
use tree_hash::{Hash256, TreeHash};
use tree_hash_derive::TreeHash;

use crate::ssz::{ceil_depth, floor_depth, length_hash, verify_merkle_proof};

#[test]
fn test_depth_calculation() {
    for x in 0..100 {
        let floor_depth = floor_depth(x);
        let ceil_depth = ceil_depth(x);
        if x == 0 {
            assert!(
                floor_depth == 0 && 0 == ceil_depth,
                "failed at num: {} (floor: {}, ceil: {})",
                x,
                floor_depth,
                ceil_depth,
            );
        } else {
            let num = x as f64;
            let ceil = 2.0f64.powf(f64::from(ceil_depth));
            let floor = 2.0f64.powf(f64::from(floor_depth));
            assert!(
                floor <= num && num <= ceil,
                "failed at num: {:.2} (floor: {:.2}, ceil: {:.2})",
                num,
                floor,
                ceil
            );
        }
    }
}

#[derive(Encode, Decode, Debug, TreeHash)]
struct Demo {
    a: u32,
    b: u32,
    c: C,
}

type C = VariableList<D, typenum::U128>;
type D = VariableList<u8, typenum::U256>;

#[derive(Debug)]
struct DemoCache {
    root: Hash256,
    a_root: Hash256,
    b_root: Hash256,
    c_root: Hash256,
    c_data_root: Hash256,
    c_depth: u32,
    d_roots: Vec<Hash256>,
}

impl Demo {
    const FIELDS_COUNT: usize = 3;
    const C_FIELDS_INDEX: usize = 2;
    const FIELDS_DEPTH: u32 = ceil_depth(Self::FIELDS_COUNT);

    fn generate_cache(self) -> DemoCache {
        let d_roots = self
            .c
            .iter()
            .map(|d| d.tree_hash_root())
            .collect::<Vec<_>>();
        let c_depth = ceil_depth(C::max_len());
        let c_data_root = {
            let tree = MerkleTree::create(&d_roots, c_depth as usize);
            tree.hash()
        };
        DemoCache {
            root: self.tree_hash_root(),
            a_root: self.a.tree_hash_root(),
            b_root: self.b.tree_hash_root(),
            c_root: self.c.tree_hash_root(),
            c_data_root,
            c_depth,
            d_roots,
        }
    }
}

impl DemoCache {
    fn c_data_root(&self) -> Hash256 {
        self.c_data_root
    }

    fn c_root(&self) -> Hash256 {
        self.c_root
    }

    fn root(&self) -> Hash256 {
        self.root
    }

    fn generate_d_proof_for_c_data(&self, index: usize) -> Vec<Hash256> {
        let depth = self.c_depth as usize;
        let tree = MerkleTree::create(&self.d_roots, depth);
        let (_, proof) = tree.generate_proof(index, depth).unwrap();
        proof
    }

    fn generate_d_proof_for_c(&self, index: usize) -> Vec<Hash256> {
        let proof_c_data = self.generate_d_proof_for_c_data(index);
        let item = length_hash(self.d_roots.len());
        let mut proof = proof_c_data;
        proof.push(item);
        proof
    }

    fn generate_d_proof(&self, index: usize) -> Vec<Hash256> {
        let proof_c = self.generate_d_proof_for_c(index);
        let leaves = vec![self.a_root, self.b_root, self.c_root];
        let depth = Demo::FIELDS_DEPTH as usize;
        let field_index = Demo::C_FIELDS_INDEX;
        let tree = MerkleTree::create(&leaves, depth);
        let (_, fields_proof) = tree.generate_proof(field_index, depth).unwrap();
        let mut proof = proof_c;
        proof.extend(fields_proof);
        proof
    }
}

#[test]
fn test_verify_merkle_proof() {
    let d_vec = (0u8..=10)
        .into_iter()
        .map(|i| VariableList::from(vec![i, i + 1, i * 2]))
        .collect::<Vec<_>>();
    let d_vec_len = d_vec.len();
    let demo = Demo {
        a: 1,
        b: 2,
        c: VariableList::from(d_vec),
    };
    let cache = demo.generate_cache();

    {
        let root = cache.c_data_root();
        let generalized_index_offset = 2usize.pow(cache.c_depth);
        for index in 0..d_vec_len {
            let leaf = &cache.d_roots[index];
            let proof = cache.generate_d_proof_for_c_data(index);
            let generalized_index = index + generalized_index_offset;
            let result = verify_merkle_proof(root, *leaf, &proof, generalized_index);
            assert!(
                result,
                "failed to verify for {}-th c-item in c-data, demo-cache: {:?}",
                index, cache
            );
        }
    }

    {
        let root = cache.c_root();
        let generalized_index_offset = 2usize.pow(cache.c_depth + 1);
        for index in 0..d_vec_len {
            let leaf = &cache.d_roots[index];
            let proof = cache.generate_d_proof_for_c(index);
            let generalized_index = index + generalized_index_offset;
            let result = verify_merkle_proof(root, *leaf, &proof, generalized_index);
            assert!(
                result,
                "failed to verify for {}-th c-item in c, demo-cache: {:?}",
                index, cache
            );
        }
    }

    let root = cache.root();
    let generalized_index_offset = {
        let tmp = 2usize.pow(cache.c_depth + 1 + Demo::FIELDS_DEPTH);
        tmp + tmp * Demo::C_FIELDS_INDEX / Demo::FIELDS_COUNT.next_power_of_two()
    };
    for index in 0..d_vec_len {
        let leaf = &cache.d_roots[index];
        let proof = cache.generate_d_proof(index);
        let generalized_index = index + generalized_index_offset;
        let result = verify_merkle_proof(root, *leaf, &proof, generalized_index);
        assert!(
            result,
            "failed to verify for {}-th c-item, demo-cache: {:?}",
            index, cache
        );
    }
}
