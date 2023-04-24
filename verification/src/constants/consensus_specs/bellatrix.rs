use crate::ssz::ceil_depth;

// From Ethereum Consensus Specs.
// Ref: https://github.com/ethereum/consensus-specs/blob/v1.2.0/specs/bellatrix/beacon-chain.md#execution
pub const MAX_BYTES_PER_TRANSACTION: usize = 1_073_741_824;
pub const MAX_TRANSACTIONS_PER_PAYLOAD: usize = 1_048_576;
pub use super::altair::SYNC_COMMITTEE_SIZE;

// There are 14 fields in `execution_payload`:
// - `receipts_root` is the 4-th field.
// - `transactions` is the 14-th field.
// Ref: https://github.com/ethereum/consensus-specs/blob/v1.2.0/specs/bellatrix/beacon-chain.md#executionpayload
pub const EXECUTION_PAYLOAD_FIELDS_COUNT: usize = 14;
pub const RECEIPTS_ROOT_IN_EXECUTION_PAYLOAD_INDEX: usize = 3;
pub const TRANSACTIONS_IN_EXECUTION_PAYLOAD_INDEX: usize = 13;
// There are 10 fields in `block_body`:
// - `execution_payload` is the 10-th field.
// Ref: https://github.com/ethereum/consensus-specs/blob/v1.2.0/specs/bellatrix/beacon-chain.md#beaconblockbody
pub const BLOCK_BODY_FIELDS_COUNT: usize = 10;
pub const EXECUTION_PAYLOAD_IN_BLOCK_BODY_INDEX: usize = 9;

pub const TRANSACTIONS_DEPTH: u32 = ceil_depth(MAX_TRANSACTIONS_PER_PAYLOAD);
pub const EXECUTION_PAYLOAD_DEPTH: u32 = ceil_depth(EXECUTION_PAYLOAD_FIELDS_COUNT);
pub const BLOCK_BODY_DEPTH: u32 = ceil_depth(BLOCK_BODY_FIELDS_COUNT);
