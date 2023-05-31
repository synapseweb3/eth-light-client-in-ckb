//! Beacon chain hard fork: [Altair].
//!
//! [Altair]: https://github.com/ethereum/consensus-specs/tree/v1.1.0/specs/altair

/// From [Fork Logic / Configuration].
///
/// [Fork Logic / Configuration]: https://github.com/ethereum/consensus-specs/blob/v1.1.0/specs/altair/fork.md#configuration
pub const FORK_EPOCH: u64 = 74240;

/// From [The Beacon Chain / Preset / Sync committee].
///
/// [The Beacon Chain / Preset / Sync committee]: https://github.com/ethereum/consensus-specs/blob/v1.1.0/specs/altair/beacon-chain.md#sync-committee
pub const SYNC_COMMITTEE_SIZE: usize = 512;
