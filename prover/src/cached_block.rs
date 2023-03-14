use eth2_types::{BeaconBlock, EthSpec, ExecPayload as _, Slot, Transaction};
use merkle_proof::MerkleTree;
use tree_hash::{Hash256, TreeHash};

use eth_light_client_in_ckb_verification::{constants::consensus_specs as specs, ssz};

#[derive(Clone)]
pub struct CachedBeaconBlock<T>
where
    T: EthSpec,
{
    body_root: Hash256,
    randao_reveal_root: Hash256,
    eth1_data_root: Hash256,
    graffiti_root: Hash256,
    proposer_slashings_root: Hash256,
    attester_slashings_root: Hash256,
    attestations_root: Hash256,
    deposits_root: Hash256,
    voluntary_exits_root: Hash256,
    sync_aggregate_root: Hash256,

    execution_payload_root: Hash256,
    parent_hash_root: Hash256,
    fee_recipient_root: Hash256,
    state_root: Hash256,
    receipts_root: Hash256,
    logs_bloom_root: Hash256,
    prev_randao_root: Hash256,
    block_number_root: Hash256,
    gas_limit_root: Hash256,
    gas_used_root: Hash256,
    timestamp_root: Hash256,
    extra_data_root: Hash256,
    base_fee_per_gas_root: Hash256,
    block_hash_root: Hash256,

    transactions_root: Hash256,
    transactions_depth: usize,
    transactions_data_root: Hash256,
    transaction_hashes: Vec<Hash256>,

    original: BeaconBlock<T>,
}

impl<T> From<BeaconBlock<T>> for CachedBeaconBlock<T>
where
    T: EthSpec,
{
    fn from(block: BeaconBlock<T>) -> Self {
        let body = block.body();
        let payload = body.execution_payload().unwrap();
        let execution_payload = &payload.execution_payload;
        let transactions = &execution_payload.transactions;

        let transaction_hashes = transactions
            .iter()
            .map(|tx| tx.tree_hash_root())
            .collect::<Vec<_>>();
        let transactions_depth = ssz::ceil_depth(T::max_transactions_per_payload()) as usize;
        let transactions_data_root =
            MerkleTree::create(&transaction_hashes, transactions_depth).hash();

        Self {
            body_root: block.body_root(),
            randao_reveal_root: body.randao_reveal().tree_hash_root(),
            eth1_data_root: body.eth1_data().tree_hash_root(),
            graffiti_root: body.graffiti().tree_hash_root(),
            proposer_slashings_root: body.proposer_slashings().tree_hash_root(),
            attester_slashings_root: body.attester_slashings().tree_hash_root(),
            attestations_root: body.attestations().tree_hash_root(),
            deposits_root: body.deposits().tree_hash_root(),
            voluntary_exits_root: body.voluntary_exits().tree_hash_root(),
            sync_aggregate_root: body.sync_aggregate().unwrap().tree_hash_root(),

            execution_payload_root: payload.tree_hash_root(),
            parent_hash_root: execution_payload.parent_hash.tree_hash_root(),
            fee_recipient_root: execution_payload.fee_recipient.tree_hash_root(),
            state_root: execution_payload.state_root.tree_hash_root(),
            receipts_root: execution_payload.receipts_root.tree_hash_root(),
            logs_bloom_root: execution_payload.logs_bloom.tree_hash_root(),
            prev_randao_root: execution_payload.prev_randao.tree_hash_root(),
            block_number_root: execution_payload.block_number.tree_hash_root(),
            gas_limit_root: execution_payload.gas_limit.tree_hash_root(),
            gas_used_root: execution_payload.gas_used.tree_hash_root(),
            timestamp_root: execution_payload.timestamp.tree_hash_root(),
            extra_data_root: execution_payload.extra_data.tree_hash_root(),
            base_fee_per_gas_root: execution_payload.base_fee_per_gas.tree_hash_root(),
            block_hash_root: execution_payload.block_hash.tree_hash_root(),

            transactions_root: transactions.tree_hash_root(),
            transactions_depth,
            transactions_data_root,
            transaction_hashes,

            original: block,
        }
    }
}

impl<T> CachedBeaconBlock<T>
where
    T: EthSpec,
{
    pub fn original(&self) -> &BeaconBlock<T> {
        &self.original
    }

    pub fn slot(&self) -> Slot {
        self.original.slot()
    }

    pub fn number(&self) -> u64 {
        self.original
            .body()
            .execution_payload()
            .unwrap()
            .block_number()
    }

    pub fn transactions_count(&self) -> usize {
        self.original
            .body()
            .execution_payload()
            .unwrap()
            .execution_payload
            .transactions
            .len()
    }

    pub fn transaction(
        &self,
        index: usize,
    ) -> Option<&Transaction<<T as EthSpec>::MaxBytesPerTransaction>> {
        self.original
            .body()
            .execution_payload()
            .unwrap()
            .execution_payload
            .transactions
            .get(index)
    }

    pub fn body_root(&self) -> Hash256 {
        self.body_root
    }

    pub fn execution_payload_root(&self) -> Hash256 {
        self.execution_payload_root
    }

    pub fn transactions_root(&self) -> Hash256 {
        self.transactions_root
    }

    pub fn transactions_data_root(&self) -> Hash256 {
        self.transactions_data_root
    }

    pub fn generate_transaction_proof_for_transactions_data(&self, index: usize) -> Vec<Hash256> {
        let depth = self.transactions_depth;
        let tree = MerkleTree::create(&self.transaction_hashes, depth);
        let (_, proof) = tree.generate_proof(index, depth).unwrap();
        proof
    }

    pub fn generate_transaction_proof_for_transactions(&self, index: usize) -> Vec<Hash256> {
        let mut proof = self.generate_transaction_proof_for_transactions_data(index);
        let item = ssz::length_hash(self.transaction_hashes.len());
        proof.push(item);
        proof
    }

    pub fn generate_transaction_proof_for_execution_payload(&self, index: usize) -> Vec<Hash256> {
        let mut proof = self.generate_transaction_proof_for_transactions(index);
        let leaves = vec![
            self.parent_hash_root,
            self.fee_recipient_root,
            self.state_root,
            self.receipts_root,
            self.logs_bloom_root,
            self.prev_randao_root,
            self.block_number_root,
            self.gas_limit_root,
            self.gas_used_root,
            self.timestamp_root,
            self.extra_data_root,
            self.base_fee_per_gas_root,
            self.block_hash_root,
            self.transactions_root,
        ];
        assert_eq!(leaves.len(), specs::EXECUTION_PAYLOAD_FIELDS_COUNT);
        let depth = specs::EXECUTION_PAYLOAD_DEPTH as usize;
        let field_index = specs::TRANSACTIONS_IN_EXECUTION_PAYLOAD_INDEX;
        let tree = MerkleTree::create(&leaves, depth);
        let (_, fields_proof) = tree.generate_proof(field_index, depth).unwrap();
        proof.extend(fields_proof);
        proof
    }

    pub fn generate_transaction_proof_for_block_body(&self, index: usize) -> Vec<Hash256> {
        let mut proof = self.generate_transaction_proof_for_execution_payload(index);
        let leaves = vec![
            self.randao_reveal_root,
            self.eth1_data_root,
            self.graffiti_root,
            self.proposer_slashings_root,
            self.attester_slashings_root,
            self.attestations_root,
            self.deposits_root,
            self.voluntary_exits_root,
            self.sync_aggregate_root,
            self.execution_payload_root,
        ];
        assert_eq!(leaves.len(), specs::BLOCK_BODY_FIELDS_COUNT);
        let depth = specs::BLOCK_BODY_DEPTH as usize;
        let field_index = specs::EXECUTION_PAYLOAD_IN_BLOCK_BODY_INDEX;
        let tree = MerkleTree::create(&leaves, depth);
        let (_, fields_proof) = tree.generate_proof(field_index, depth).unwrap();
        proof.extend(fields_proof);
        proof
    }

    pub fn generate_receipts_root_proof_for_execution_payload(&self) -> Vec<Hash256> {
        let leaves = vec![
            self.parent_hash_root,
            self.fee_recipient_root,
            self.state_root,
            self.receipts_root,
            self.logs_bloom_root,
            self.prev_randao_root,
            self.block_number_root,
            self.gas_limit_root,
            self.gas_used_root,
            self.timestamp_root,
            self.extra_data_root,
            self.base_fee_per_gas_root,
            self.block_hash_root,
            self.transactions_root,
        ];
        assert_eq!(leaves.len(), specs::EXECUTION_PAYLOAD_FIELDS_COUNT);
        let depth = specs::EXECUTION_PAYLOAD_DEPTH as usize;
        let field_index = specs::RECEIPTS_ROOT_IN_EXECUTION_PAYLOAD_INDEX;
        let tree = MerkleTree::create(&leaves, depth);
        let (_, proof) = tree.generate_proof(field_index, depth).unwrap();
        proof
    }

    pub fn generate_receipts_root_proof_for_block_body(&self) -> Vec<Hash256> {
        let mut proof = self.generate_receipts_root_proof_for_execution_payload();
        let leaves = vec![
            self.randao_reveal_root,
            self.eth1_data_root,
            self.graffiti_root,
            self.proposer_slashings_root,
            self.attester_slashings_root,
            self.attestations_root,
            self.deposits_root,
            self.voluntary_exits_root,
            self.sync_aggregate_root,
            self.execution_payload_root,
        ];
        assert_eq!(leaves.len(), specs::BLOCK_BODY_FIELDS_COUNT);
        let depth = specs::BLOCK_BODY_DEPTH as usize;
        let field_index = specs::EXECUTION_PAYLOAD_IN_BLOCK_BODY_INDEX;
        let tree = MerkleTree::create(&leaves, depth);
        let (_, fields_proof) = tree.generate_proof(field_index, depth).unwrap();
        proof.extend(fields_proof);
        proof
    }
}
