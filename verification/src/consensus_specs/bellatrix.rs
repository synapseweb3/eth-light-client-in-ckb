//! Beacon chain hard fork: [Bellatrix].
//!
//! [Bellatrix]: https://github.com/ethereum/consensus-specs/tree/v1.2.0/specs/bellatrix

use super::altair as previous_fork;

/// From [Fork Logic / Configuration].
///
/// [Fork Logic / Configuration]: https://github.com/ethereum/consensus-specs/blob/v1.2.0/specs/bellatrix/fork.md#configuration
pub const FORK_EPOCH: u64 = 144896;

pub use previous_fork::SYNC_COMMITTEE_SIZE;

/// From [The Beacon Chain / Preset / Execution].
///
/// [The Beacon Chain / Preset / Execution]: https://github.com/ethereum/consensus-specs/blob/v1.2.0/specs/bellatrix/beacon-chain.md#execution
pub const MAX_BYTES_PER_TRANSACTION: usize = 1_073_741_824;

/// From [The Beacon Chain / Preset / Execution].
///
/// [The Beacon Chain / Preset / Execution]: https://github.com/ethereum/consensus-specs/blob/v1.2.0/specs/bellatrix/beacon-chain.md#execution
pub const MAX_TRANSACTIONS_PER_PAYLOAD: usize = 1_048_576;

/// Constants for containers.
pub mod containers {
    use crate::ssz::ceil_depth;

    use super::MAX_TRANSACTIONS_PER_PAYLOAD;

    /// There are 14 fields in [`ExecutionPayload`].
    ///
    /// [`ExecutionPayload`]: https://github.com/ethereum/consensus-specs/blob/v1.2.0/specs/bellatrix/beacon-chain.md#executionpayload
    pub const EXECUTION_PAYLOAD_FIELDS_COUNT: usize = 14;

    /// `receipts_root` is the 4-th field in [`ExecutionPayload`].
    ///
    /// [`ExecutionPayload`]: https://github.com/ethereum/consensus-specs/blob/v1.2.0/specs/bellatrix/beacon-chain.md#executionpayload
    pub const RECEIPTS_ROOT_IN_EXECUTION_PAYLOAD_INDEX: usize = 3;

    /// `transactions` is the 14-th field in [`ExecutionPayload`].
    ///
    /// [`ExecutionPayload`]: https://github.com/ethereum/consensus-specs/blob/v1.2.0/specs/bellatrix/beacon-chain.md#executionpayload
    pub const TRANSACTIONS_IN_EXECUTION_PAYLOAD_INDEX: usize = 13;

    /// There are 10 fields in [`BeaconBlockBody`].
    ///
    /// [`BeaconBlockBody`]: https://github.com/ethereum/consensus-specs/blob/v1.2.0/specs/bellatrix/beacon-chain.md#beaconblockbody
    pub const BLOCK_BODY_FIELDS_COUNT: usize = 10;

    /// `execution_payload` is the 10-th field in [`BeaconBlockBody`].
    ///
    /// [`BeaconBlockBody`]: https://github.com/ethereum/consensus-specs/blob/v1.2.0/specs/bellatrix/beacon-chain.md#beaconblockbody
    pub const EXECUTION_PAYLOAD_IN_BLOCK_BODY_INDEX: usize = 9;

    /// The depth of [`MAX_TRANSACTIONS_PER_PAYLOAD`].
    pub const TRANSACTIONS_DEPTH: u32 = ceil_depth(MAX_TRANSACTIONS_PER_PAYLOAD);
    /// The depth of [`EXECUTION_PAYLOAD_FIELDS_COUNT`].
    pub const EXECUTION_PAYLOAD_DEPTH: u32 = ceil_depth(EXECUTION_PAYLOAD_FIELDS_COUNT);
    /// The depth of [`BLOCK_BODY_FIELDS_COUNT`].
    pub const BLOCK_BODY_DEPTH: u32 = ceil_depth(BLOCK_BODY_FIELDS_COUNT);
}

define_generalized_index_mod!(super::containers);
