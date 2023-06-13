use crate::consensus_specs::{forks, helpers};

pub const fn get_generalized_index_of_receipts_root_in_block_body(slot: u64) -> usize {
    if slot < helpers::compute_start_slot_at_epoch(forks::capella::FORK_EPOCH) {
        forks::bellatrix::generalized_index::RECEIPTS_ROOT_IN_BLOCK_BODY
    } else {
        forks::capella::generalized_index::RECEIPTS_ROOT_IN_BLOCK_BODY
    }
}

pub const fn get_generalized_index_of_transaction_in_block_body_offset(slot: u64) -> usize {
    if slot < helpers::compute_start_slot_at_epoch(forks::capella::FORK_EPOCH) {
        forks::bellatrix::generalized_index::TRANSACTION_IN_BLOCK_BODY_OFFSET
    } else {
        forks::capella::generalized_index::TRANSACTION_IN_BLOCK_BODY_OFFSET
    }
}

pub const fn get_depth_and_index_from_current_sync_committee_index(slot: u64) -> (u32, usize) {
    if slot < helpers::compute_start_slot_at_epoch(forks::capella::FORK_EPOCH) {
        const INDEX: usize =
            forks::bellatrix::generalized_index::beacon_state::CURRENT_SYNC_COMMITTEE_INDEX;
        (helpers::floorlog2(INDEX), helpers::get_subtree_index(INDEX))
    } else {
        const INDEX: usize =
            forks::capella::generalized_index::beacon_state::CURRENT_SYNC_COMMITTEE_INDEX;
        (helpers::floorlog2(INDEX), helpers::get_subtree_index(INDEX))
    }
}

pub const fn get_depth_and_index_from_next_sync_committee_index(slot: u64) -> (u32, usize) {
    if slot < helpers::compute_start_slot_at_epoch(forks::capella::FORK_EPOCH) {
        const INDEX: usize =
            forks::bellatrix::generalized_index::beacon_state::NEXT_SYNC_COMMITTEE_INDEX;
        (helpers::floorlog2(INDEX), helpers::get_subtree_index(INDEX))
    } else {
        const INDEX: usize =
            forks::capella::generalized_index::beacon_state::NEXT_SYNC_COMMITTEE_INDEX;
        (helpers::floorlog2(INDEX), helpers::get_subtree_index(INDEX))
    }
}

pub const fn get_depth_and_index_from_finalized_root_index(slot: u64) -> (u32, usize) {
    if slot < helpers::compute_start_slot_at_epoch(forks::capella::FORK_EPOCH) {
        const INDEX: usize =
            forks::bellatrix::generalized_index::beacon_state::FINALIZED_ROOT_INDEX;
        (helpers::floorlog2(INDEX), helpers::get_subtree_index(INDEX))
    } else {
        const INDEX: usize = forks::capella::generalized_index::beacon_state::FINALIZED_ROOT_INDEX;
        (helpers::floorlog2(INDEX), helpers::get_subtree_index(INDEX))
    }
}
