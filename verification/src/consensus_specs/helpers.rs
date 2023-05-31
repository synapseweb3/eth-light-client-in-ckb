//! Helper functions.

use super::phase0;

/// See [`compute_epoch_at_slot`].
///
/// [`compute_epoch_at_slot`]: https://github.com/ethereum/consensus-specs/blob/v1.0.0/specs/phase0/beacon-chain.md#compute_epoch_at_slot
pub const fn compute_epoch_at_slot(slot: u64) -> u64 {
    slot / phase0::SLOTS_PER_EPOCH
}

/// See [`compute_start_slot_at_epoch`].
///
/// [`compute_start_slot_at_epoch`]: https://github.com/ethereum/consensus-specs/blob/v1.0.0/specs/phase0/beacon-chain.md#compute_start_slot_at_epoch
pub const fn compute_start_slot_at_epoch(epoch: u64) -> u64 {
    epoch * phase0::SLOTS_PER_EPOCH
}
