use std::sync::Arc;

use cita_trie::{MemoryDB, PatriciaTrie, Trie};
use ethers_core::{
    types::TransactionReceipt,
    utils::rlp::{encode, Encodable as _},
};
use hasher::HasherKeccak;
use tree_hash::Hash256;

pub struct Receipts {
    root: Hash256,
    trie: PatriciaTrie<MemoryDB, HasherKeccak>,
    original: Vec<TransactionReceipt>,
}

impl From<Vec<TransactionReceipt>> for Receipts {
    fn from(receipts: Vec<TransactionReceipt>) -> Self {
        let memdb = Arc::new(MemoryDB::new(true));
        let hasher = Arc::new(HasherKeccak::new());
        let mut trie = PatriciaTrie::new(Arc::clone(&memdb), Arc::clone(&hasher));
        for (i, receipt) in receipts.iter().enumerate() {
            let key = encode(&i);
            let value = encode_receipt(receipt);
            trie.insert(key.to_vec(), value).unwrap();
        }
        let trie_root = trie.root().unwrap();
        let root = Hash256::from_slice(&trie_root[0..32]);
        Self {
            root,
            trie,
            original: receipts,
        }
    }
}

impl Receipts {
    pub fn original(&self) -> &[TransactionReceipt] {
        &self.original
    }

    pub fn root(&self) -> Hash256 {
        self.root
    }

    pub fn generate_proof(&self, index: usize) -> Vec<Vec<u8>> {
        let key = encode(&index);
        self.trie.get_proof(&key).unwrap()
    }
}

// NOTE The implementation in `ethers` is incorrect.
// Ref:
// - https://eips.ethereum.org/EIPS/eip-2718#receipts
// - https://github.com/gakonst/ethers-rs/blob/v1.0/ethers-core/src/types/transaction/response.rs#L443-L451
pub fn encode_receipt(receipt: &TransactionReceipt) -> Vec<u8> {
    let legacy_receipt_encoded = receipt.rlp_bytes();
    if let Some(tx_type) = receipt.transaction_type {
        let tx_type = tx_type.as_u64();
        if tx_type == 0 {
            legacy_receipt_encoded.to_vec()
        } else {
            [&tx_type.to_be_bytes()[7..8], &legacy_receipt_encoded].concat()
        }
    } else {
        legacy_receipt_encoded.to_vec()
    }
}
