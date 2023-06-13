//! Beacon chain hard fork: [Altair].
//!
//! [Altair]: https://github.com/ethereum/consensus-specs/tree/v1.1.0/specs/altair

/// From [Fork Logic / Configuration].
///
/// [Fork Logic / Configuration]: https://github.com/ethereum/consensus-specs/blob/v1.1.0/specs/altair/fork.md#configuration
pub const FORK_EPOCH: u64 = 74240;

/// From [Fork Logic / Configuration].
///
/// [Fork Logic / Configuration]: https://github.com/ethereum/consensus-specs/blob/v1.1.0/specs/altair/fork.md#configuration
pub const FORK_VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];

/// From [The Beacon Chain / Constants / Domain types].
///
/// [The Beacon Chain / Constants / Domain types]: https://github.com/ethereum/consensus-specs/blob/v1.1.0/specs/altair/beacon-chain.md#domain-types
pub const DOMAIN_SYNC_COMMITTEE: [u8; 4] = [0x07, 0x00, 0x00, 0x00];

/// From [The Beacon Chain / Preset / Sync committee].
///
/// [The Beacon Chain / Preset / Sync committee]: https://github.com/ethereum/consensus-specs/blob/v1.1.0/specs/altair/beacon-chain.md#sync-committee
pub const SYNC_COMMITTEE_SIZE: usize = 512;

/// From [The Beacon Chain / Preset / Sync committee].
///
/// [The Beacon Chain / Preset / Sync committee]: https://github.com/ethereum/consensus-specs/blob/v1.1.0/specs/altair/beacon-chain.md#sync-committee
pub const EPOCHS_PER_SYNC_COMMITTEE_PERIOD: u64 = 256;

define_generalized_index_mod!(|| {
    pub mod beacon_state {
        //! Pre-computed generalized indexes for [BeaconState].
        //!
        //! [BeaconState]: https://github.com/ethereum/consensus-specs/blob/v1.1.0/specs/altair/beacon-chain.md#beaconstate

        /// From [Light Client / Sync Protocol / Constants].
        ///
        /// [Light Client / Sync Protocol / Constants]: https://github.com/ethereum/consensus-specs/blob/v1.2.0/specs/altair/light-client/sync-protocol.md#constants
        pub const FINALIZED_ROOT_INDEX: usize = 105;

        /// From [Light Client / Sync Protocol / Constants].
        ///
        /// [Light Client / Sync Protocol / Constants]: https://github.com/ethereum/consensus-specs/blob/v1.2.0/specs/altair/light-client/sync-protocol.md#constants
        pub const CURRENT_SYNC_COMMITTEE_INDEX: usize = 54;

        /// From [Light Client / Sync Protocol / Constants].
        ///
        /// [Light Client / Sync Protocol / Constants]: https://github.com/ethereum/consensus-specs/blob/v1.2.0/specs/altair/light-client/sync-protocol.md#constants
        pub const NEXT_SYNC_COMMITTEE_INDEX: usize = 55;
    }
});
