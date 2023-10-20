use std::sync::Arc;

use cita_trie::{MemoryDB, PatriciaTrie, Trie};
use ethers_core::{
    types::{TransactionReceipt, U256},
    utils::rlp::{encode, RlpStream},
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

    pub fn encode_data(&self, index: usize) -> Vec<u8> {
        encode_receipt(&self.original[index])
    }
}

/// Encode a transaction receipt into bytes.
///
/// According to [`EIP-2718`]:
/// - `Receipt` is either `TransactionType || ReceiptPayload` or
///   `LegacyReceipt`.
/// - `LegacyReceipt` is kept to be RLP encoded bytes; it is `rlp([status,
///   cumulativeGasUsed, logsBloom, logs])`.
/// - `ReceiptPayload` is an opaque byte array whose interpretation is
///   dependent on the `TransactionType` and defined in future EIPs.
///   - As [`EIP-2930`] defined: if `TransactionType` is `1`,
///     `ReceiptPayload` is `rlp([status, cumulativeGasUsed, logsBloom,
///     logs])`.
///   - As [`EIP-1559`] defined: if `TransactionType` is `2`,
///     `ReceiptPayload` is `rlp([status, cumulative_transaction_gas_used,
///     logs_bloom, logs])`.
///
/// [`EIP-2718`]: https://eips.ethereum.org/EIPS/eip-2718#receipts
/// [`EIP-2930`]: https://eips.ethereum.org/EIPS/eip-2930#parameters
/// [`EIP-1559`]: https://eips.ethereum.org/EIPS/eip-1559#specification
pub fn encode_receipt(receipt: &TransactionReceipt) -> Vec<u8> {
    let used_gas = U256::from(receipt.cumulative_gas_used);
    let legacy_receipt = {
        let mut rlp = RlpStream::new();
        rlp.begin_list(4);
        rlp.append(&receipt.status.unwrap());
        rlp.append(&used_gas);
        rlp.append(&receipt.logs_bloom);
        rlp.append_list(&receipt.logs);
        rlp.out().freeze()
    };

    let tx_type: u64 = match receipt.transaction_type {
        Some(tx_type) => tx_type.try_into().unwrap(),
        None => 0,
    };
    println!(
        "status: {:?}, used_gas: {}, tx_type: {}",
        receipt.status.unwrap(),
        used_gas,
        tx_type
    );
    match tx_type {
        x if x == 0x01 || x == 0x02 => [&x.to_be_bytes()[7..], &legacy_receipt].concat().into(),
        _ => legacy_receipt.into(), // legacy (0x00) or undefined type
    }
}

#[cfg(test)]
mod tests {
    use ethers_core::{
        types::{Bloom, Log, TransactionReceipt, H256, U256, U64},
        utils::keccak256,
    };

    use crate::encode_receipt;

    pub fn logs_bloom<'a, I>(logs: I) -> Bloom
    where
        I: Iterator<Item = &'a Log>,
    {
        let mut bloom = Bloom::zero();

        for log in logs {
            m3_2048(&mut bloom, log.address.as_bytes());
            for topic in log.topics.iter() {
                m3_2048(&mut bloom, topic.as_bytes());
            }
        }
        bloom
    }

    pub struct Hasher;
    impl Hasher {
        pub fn digest<B: AsRef<[u8]>>(bytes: B) -> H256 {
            if bytes.as_ref().is_empty() {
                return NIL_DATA;
            }

            H256(keccak256(bytes))
        }
    }

    pub const NIL_DATA: H256 = H256([
        0xc5, 0xd2, 0x46, 0x01, 0x86, 0xf7, 0x23, 0x3c, 0x92, 0x7e, 0x7d, 0xb2, 0xdc, 0xc7, 0x03,
        0xc0, 0xe5, 0x00, 0xb6, 0x53, 0xca, 0x82, 0x27, 0x3b, 0x7b, 0xfa, 0xd8, 0x04, 0x5d, 0x85,
        0xa4, 0x70,
    ]);
    const BLOOM_BYTE_LENGTH: usize = 256;

    fn m3_2048(bloom: &mut Bloom, x: &[u8]) {
        let hash = Hasher::digest(x).0;
        for i in [0, 2, 4] {
            let bit = (hash[i + 1] as usize + ((hash[i] as usize) << 8)) & 0x7FF;
            bloom.0[BLOOM_BYTE_LENGTH - 1 - bit / 8] |= 1 << (bit % 8);
        }
    }

    #[test]
    fn test_receipt() {
        let mut receipt = TransactionReceipt::default();
        receipt.transaction_hash = H256::from([0u8; 32]);
        receipt.transaction_index = 0.into();
        receipt.cumulative_gas_used = U256::from(10);
        receipt.transaction_type = Some(U64::from(2));
        receipt.status = Some(U64::from(1));
        let logs = vec![Log::default()];
        receipt.logs_bloom = logs_bloom(logs.iter());
        receipt.logs = logs;

        let receipt = encode_receipt(&receipt);

        let reference_encode: Vec<u8> = [
            2, 249, 1, 30, 1, 10, 185, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 216, 215, 148, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 192, 128,
        ]
        .to_vec();
        assert_eq!(receipt, reference_encode);
    }
}
