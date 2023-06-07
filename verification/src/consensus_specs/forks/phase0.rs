//! Beacon chain: [Phase 0].
//!
//! [Phase 0]: https://github.com/ethereum/consensus-specs/tree/v1.0.0/specs/phase0

/// From [The Beacon Chain / Configuration / Initial values].
///
/// [The Beacon Chain / Configuration/ Initial values]: https://github.com/ethereum/consensus-specs/blob/v1.0.0/specs/phase0/beacon-chain.md#initial-values
pub const GENESIS_FORK_VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];

/// From [The Beacon Chain / Configuration / Time parameters].
///
/// [The Beacon Chain / Configuration/ Time parameters]: https://github.com/ethereum/consensus-specs/blob/v1.0.0/specs/phase0/beacon-chain.md#time-parameters
pub const SLOTS_PER_EPOCH: u64 = 32;
