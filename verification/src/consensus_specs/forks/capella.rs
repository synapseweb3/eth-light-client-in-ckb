//! Beacon chain hard fork: [Capella].
//!
//! [Capella]: https://github.com/ethereum/consensus-specs/tree/v1.3.0/specs/capella

use super::bellatrix as previous_fork;

/// From [Fork Logic / Configuration].
///
/// [Fork Logic / Configuration]: https://github.com/ethereum/consensus-specs/blob/v1.3.0/specs/capella/fork.md#configuration
pub const FORK_EPOCH: u64 = 194048;

/// From [Fork Logic / Configuration].
///
/// [Fork Logic / Configuration]: https://github.com/ethereum/consensus-specs/blob/v1.3.0/specs/capella/fork.md#configuration
pub const FORK_VERSION: [u8; 4] = [0x03, 0x00, 0x00, 0x00];

/// Constants for containers.
pub mod containers {
    use crate::utilities::ssz::ceil_depth;

    use super::previous_fork::MAX_TRANSACTIONS_PER_PAYLOAD;

    /// There are 15 fields in [`ExecutionPayload`].
    ///
    /// [`ExecutionPayload`]: https://github.com/ethereum/consensus-specs/blob/v1.3.0/specs/capella/beacon-chain.md#executionpayload
    pub const EXECUTION_PAYLOAD_FIELDS_COUNT: usize = 15;

    /// `receipts_root` is the 4-th field in [`ExecutionPayload`].
    ///
    /// [`ExecutionPayload`]: https://github.com/ethereum/consensus-specs/blob/v1.3.0/specs/capella/beacon-chain.md#executionpayload
    pub const RECEIPTS_ROOT_IN_EXECUTION_PAYLOAD_INDEX: usize = 3;

    /// `transactions` is the 14-th field in [`ExecutionPayload`].
    ///
    /// [`ExecutionPayload`]: https://github.com/ethereum/consensus-specs/blob/v1.3.0/specs/capella/beacon-chain.md#executionpayload
    pub const TRANSACTIONS_IN_EXECUTION_PAYLOAD_INDEX: usize = 13;

    /// There are 11 fields in [`BeaconBlockBody`].
    ///
    /// [`BeaconBlockBody`]: https://github.com/ethereum/consensus-specs/blob/v1.3.0/specs/capella/beacon-chain.md#beaconblockbody
    pub const BLOCK_BODY_FIELDS_COUNT: usize = 11;

    /// `execution_payload` is the 10-th field in [`BeaconBlockBody`].
    ///
    /// [`BeaconBlockBody`]: https://github.com/ethereum/consensus-specs/blob/v1.3.0/specs/capella/beacon-chain.md#beaconblockbody
    pub const EXECUTION_PAYLOAD_IN_BLOCK_BODY_INDEX: usize = 9;

    /// The depth of [`MAX_TRANSACTIONS_PER_PAYLOAD`].
    pub const TRANSACTIONS_DEPTH: u32 = ceil_depth(MAX_TRANSACTIONS_PER_PAYLOAD);
    /// The depth of [`EXECUTION_PAYLOAD_FIELDS_COUNT`].
    pub const EXECUTION_PAYLOAD_DEPTH: u32 = ceil_depth(EXECUTION_PAYLOAD_FIELDS_COUNT);
    /// The depth of [`BLOCK_BODY_FIELDS_COUNT`].
    pub const BLOCK_BODY_DEPTH: u32 = ceil_depth(BLOCK_BODY_FIELDS_COUNT);
}

define_generalized_index_mod!(|super::containers| {
    pub mod beacon_state {
        //! Pre-computed generalized indexes for [BeaconState].
        //!
        //! [BeaconState]: https://github.com/ethereum/consensus-specs/blob/v1.3.0/specs/capella/beacon-chain.md#beaconstate

        use super::super::previous_fork::generalized_index::beacon_state as previous_fork;

        /// From [Light Client / Sync Protocol / Constants].
        ///
        /// [Light Client / Sync Protocol / Constants]: https://github.com/ethereum/consensus-specs/blob/v1.3.0/specs/altair/light-client/sync-protocol.md#constants
        pub const FINALIZED_ROOT_INDEX: usize = previous_fork::FINALIZED_ROOT_INDEX;

        /// From [Light Client / Sync Protocol / Constants].
        ///
        /// [Light Client / Sync Protocol / Constants]: https://github.com/ethereum/consensus-specs/blob/v1.3.0/specs/altair/light-client/sync-protocol.md#constants
        pub const CURRENT_SYNC_COMMITTEE_INDEX: usize = previous_fork::CURRENT_SYNC_COMMITTEE_INDEX;

        /// From [Light Client / Sync Protocol / Constants].
        ///
        /// [Light Client / Sync Protocol / Constants]: https://github.com/ethereum/consensus-specs/blob/v1.3.0/specs/altair/light-client/sync-protocol.md#constants
        pub const NEXT_SYNC_COMMITTEE_INDEX: usize = previous_fork::NEXT_SYNC_COMMITTEE_INDEX;
    }
});
