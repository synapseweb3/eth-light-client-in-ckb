use eth2_types::{BeaconBlock, EthSpec, ExecPayload as _, MainnetEthSpec, Slot, Transaction};
use merkle_proof::MerkleTree;
use tree_hash::{Hash256, TreeHash};

use eth_light_client_in_ckb_verification::{
    consensus_specs::{forks, helpers},
    utilities::ssz,
};

#[derive(Clone)]
pub struct CachedBeaconBlock {
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

    // Capella
    withdrawals_root: Option<Hash256>,
    bls_to_execution_changes_root: Option<Hash256>,

    original: BeaconBlock<MainnetEthSpec>,
}

impl From<BeaconBlock<MainnetEthSpec>> for CachedBeaconBlock {
    fn from(block: BeaconBlock<MainnetEthSpec>) -> Self {
        let body = block.body();
        let payload = body.execution_payload().unwrap();
        let payload_header = payload.to_execution_payload_header();
        let transactions = payload.transactions().unwrap();

        let transaction_hashes = transactions
            .iter()
            .map(|tx| tx.tree_hash_root())
            .collect::<Vec<_>>();
        let transactions_depth =
            ssz::ceil_depth(MainnetEthSpec::max_transactions_per_payload()) as usize;
        let transactions_data_root =
            MerkleTree::create(&transaction_hashes, transactions_depth).hash();

        let withdrawals_root = payload.withdrawals_root().ok();
        let bls_to_execution_changes_root = body
            .bls_to_execution_changes()
            .ok()
            .map(TreeHash::tree_hash_root);

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
            parent_hash_root: payload.parent_hash().tree_hash_root(),
            fee_recipient_root: payload.fee_recipient().tree_hash_root(),
            state_root: payload_header.state_root().tree_hash_root(),
            receipts_root: payload_header.receipts_root().tree_hash_root(),
            logs_bloom_root: payload_header.logs_bloom().tree_hash_root(),
            prev_randao_root: payload.prev_randao().tree_hash_root(),
            block_number_root: payload.block_number().tree_hash_root(),
            gas_limit_root: payload.gas_limit().tree_hash_root(),
            gas_used_root: payload_header.gas_used().tree_hash_root(),
            timestamp_root: payload.timestamp().tree_hash_root(),
            extra_data_root: payload_header.extra_data().tree_hash_root(),
            base_fee_per_gas_root: payload_header.base_fee_per_gas().tree_hash_root(),
            block_hash_root: payload.block_hash().tree_hash_root(),

            transactions_root: transactions.tree_hash_root(),
            transactions_depth,
            transactions_data_root,
            transaction_hashes,

            withdrawals_root,
            bls_to_execution_changes_root,

            original: block,
        }
    }
}

impl CachedBeaconBlock {
    pub fn original(&self) -> &BeaconBlock<MainnetEthSpec> {
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
            .transactions()
            .unwrap()
            .len()
    }

    pub fn transaction(
        &self,
        index: usize,
    ) -> Option<Transaction<<MainnetEthSpec as EthSpec>::MaxBytesPerTransaction>> {
        self.original
            .body()
            .execution_payload()
            .unwrap()
            .transactions()
            .unwrap()
            .get(index)
            .cloned()
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
        let mut leaves = vec![
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
        let (depth, field_index) =
            if self.slot() < helpers::compute_start_slot_at_epoch(forks::capella::FORK_EPOCH) {
                use forks::bellatrix::containers;
                assert_eq!(leaves.len(), containers::EXECUTION_PAYLOAD_FIELDS_COUNT);
                let depth = containers::EXECUTION_PAYLOAD_DEPTH as usize;
                let field_index = containers::TRANSACTIONS_IN_EXECUTION_PAYLOAD_INDEX;
                (depth, field_index)
            } else {
                use forks::capella::containers;
                leaves.push(self.withdrawals_root.unwrap());
                assert_eq!(leaves.len(), containers::EXECUTION_PAYLOAD_FIELDS_COUNT);
                let depth = containers::EXECUTION_PAYLOAD_DEPTH as usize;
                let field_index = containers::TRANSACTIONS_IN_EXECUTION_PAYLOAD_INDEX;
                (depth, field_index)
            };
        let tree = MerkleTree::create(&leaves, depth);
        let (_, fields_proof) = tree.generate_proof(field_index, depth).unwrap();
        proof.extend(fields_proof);
        proof
    }

    pub fn generate_transaction_proof_for_block_body(&self, index: usize) -> Vec<Hash256> {
        let mut proof = self.generate_transaction_proof_for_execution_payload(index);
        let mut leaves = vec![
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
        let (depth, field_index) =
            if self.slot() < helpers::compute_start_slot_at_epoch(forks::capella::FORK_EPOCH) {
                use forks::bellatrix::containers;
                assert_eq!(leaves.len(), containers::BLOCK_BODY_FIELDS_COUNT);
                let depth = containers::BLOCK_BODY_DEPTH as usize;
                let field_index = containers::EXECUTION_PAYLOAD_IN_BLOCK_BODY_INDEX;
                (depth, field_index)
            } else {
                use forks::capella::containers;
                leaves.push(self.bls_to_execution_changes_root.unwrap());
                assert_eq!(leaves.len(), containers::BLOCK_BODY_FIELDS_COUNT);
                let depth = containers::BLOCK_BODY_DEPTH as usize;
                let field_index = containers::EXECUTION_PAYLOAD_IN_BLOCK_BODY_INDEX;
                (depth, field_index)
            };
        let tree = MerkleTree::create(&leaves, depth);
        let (_, fields_proof) = tree.generate_proof(field_index, depth).unwrap();
        proof.extend(fields_proof);
        proof
    }

    pub fn generate_receipts_root_proof_for_execution_payload(&self) -> Vec<Hash256> {
        let mut leaves = vec![
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
        let (depth, field_index) =
            if self.slot() < helpers::compute_start_slot_at_epoch(forks::capella::FORK_EPOCH) {
                use forks::bellatrix::containers;
                assert_eq!(leaves.len(), containers::EXECUTION_PAYLOAD_FIELDS_COUNT);
                let depth = containers::EXECUTION_PAYLOAD_DEPTH as usize;
                let field_index = containers::RECEIPTS_ROOT_IN_EXECUTION_PAYLOAD_INDEX;
                (depth, field_index)
            } else {
                use forks::capella::containers;
                leaves.push(self.withdrawals_root.unwrap());
                assert_eq!(leaves.len(), containers::EXECUTION_PAYLOAD_FIELDS_COUNT);
                let depth = containers::EXECUTION_PAYLOAD_DEPTH as usize;
                let field_index = containers::RECEIPTS_ROOT_IN_EXECUTION_PAYLOAD_INDEX;
                (depth, field_index)
            };

        let tree = MerkleTree::create(&leaves, depth);
        let (_, proof) = tree.generate_proof(field_index, depth).unwrap();
        proof
    }

    pub fn generate_receipts_root_proof_for_block_body(&self) -> Vec<Hash256> {
        let mut proof = self.generate_receipts_root_proof_for_execution_payload();
        let mut leaves = vec![
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
        let (depth, field_index) =
            if self.slot() < helpers::compute_start_slot_at_epoch(forks::capella::FORK_EPOCH) {
                use forks::bellatrix::containers;
                assert_eq!(leaves.len(), containers::BLOCK_BODY_FIELDS_COUNT);
                let depth = containers::BLOCK_BODY_DEPTH as usize;
                let field_index = containers::EXECUTION_PAYLOAD_IN_BLOCK_BODY_INDEX;
                (depth, field_index)
            } else {
                use forks::capella::containers;
                leaves.push(self.bls_to_execution_changes_root.unwrap());
                assert_eq!(leaves.len(), containers::BLOCK_BODY_FIELDS_COUNT);
                let depth = containers::BLOCK_BODY_DEPTH as usize;
                let field_index = containers::EXECUTION_PAYLOAD_IN_BLOCK_BODY_INDEX;
                (depth, field_index)
            };
        let tree = MerkleTree::create(&leaves, depth);
        let (_, fields_proof) = tree.generate_proof(field_index, depth).unwrap();
        proof.extend(fields_proof);
        proof
    }
}
