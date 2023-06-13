//! Helper functions.

use crate::{consensus_specs::forks, utilities::ssz::floor_depth};

/// See [`compute_epoch_at_slot`].
///
/// [`compute_epoch_at_slot`]: https://github.com/ethereum/consensus-specs/blob/v1.0.0/specs/phase0/beacon-chain.md#compute_epoch_at_slot
pub const fn compute_epoch_at_slot(slot: u64) -> u64 {
    slot / forks::phase0::SLOTS_PER_EPOCH
}

/// See [`compute_start_slot_at_epoch`].
///
/// [`compute_start_slot_at_epoch`]: https://github.com/ethereum/consensus-specs/blob/v1.0.0/specs/phase0/beacon-chain.md#compute_start_slot_at_epoch
pub const fn compute_start_slot_at_epoch(epoch: u64) -> u64 {
    epoch * forks::phase0::SLOTS_PER_EPOCH
}

/// See [Sync Committee].
///
/// [Sync Committee]: https://github.com/ethereum/consensus-specs/blob/v1.1.0/specs/altair/validator.md#sync-committee
pub const fn compute_sync_committee_period(epoch: u64) -> u64 {
    epoch / forks::altair::EPOCHS_PER_SYNC_COMMITTEE_PERIOD
}

/// See [`compute_sync_committee_period_at_slot`].
///
/// [`compute_sync_committee_period_at_slot`]: https://github.com/ethereum/consensus-specs/blob/v1.2.0/specs/altair/light-client/sync-protocol.md#compute_sync_committee_period_at_slot
pub const fn compute_sync_committee_period_at_slot(slot: u64) -> u64 {
    compute_sync_committee_period(compute_epoch_at_slot(slot))
}

/// Returns the fork version at the given `epoch`.
///
/// See [`compute_fork_version`].
///
/// [`compute_fork_version`]: https://github.com/ethereum/consensus-specs/blob/v1.3.0/specs/capella/fork.md#modified-compute_fork_version
pub const fn compute_fork_version(epoch: u64) -> [u8; 4] {
    if epoch >= forks::capella::FORK_EPOCH {
        forks::capella::FORK_VERSION
    } else if epoch >= forks::bellatrix::FORK_EPOCH {
        forks::bellatrix::FORK_VERSION
    } else if epoch >= forks::altair::FORK_EPOCH {
        forks::altair::FORK_VERSION
    } else {
        forks::phase0::GENESIS_FORK_VERSION
    }
}

/// Returns the fork version at the given `slot`.
///
/// See [`compute_fork_version`].
pub const fn compute_fork_version_at_slot(slot: u64) -> [u8; 4] {
    compute_fork_version(compute_epoch_at_slot(slot))
}

/// See [`get_subtree_index`].
///
/// [`get_subtree_index`]: https://github.com/ethereum/consensus-specs/blob/v1.2.0/specs/altair/light-client/sync-protocol.md#get_subtree_index
pub const fn get_subtree_index(index: usize) -> usize {
    index % (2usize.pow(floorlog2(index)))
}

/// See [`floorlog2`].
///
/// [`floorlog2`]: https://github.com/ethereum/consensus-specs/blob/v1.2.0/setup.py#L59
pub const fn floorlog2(num: usize) -> u32 {
    floor_depth(num)
}
