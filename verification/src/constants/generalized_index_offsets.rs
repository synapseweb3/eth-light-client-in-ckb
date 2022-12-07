use crate::constants::consensus_specs as specs;

pub const TRANSACTION_IN_TRANSACTIONS_DATA: usize = 2usize.pow(specs::TRANSACTIONS_DEPTH);

pub const TRANSACTION_IN_TRANSACTIONS: usize = 2usize.pow(specs::TRANSACTIONS_DEPTH + 1);

pub const TRANSACTION_IN_EXECUTION_PAYLOAD: usize = {
    const DEPTH: u32 = specs::TRANSACTIONS_DEPTH + 1 + specs::EXECUTION_PAYLOAD_DEPTH;
    const SIZE: usize = 2usize.pow(DEPTH);

    const FIELDS_COUNT_1: usize = specs::EXECUTION_PAYLOAD_FIELDS_COUNT.next_power_of_two();
    const FIELD_INDEX_1: usize = specs::TRANSACTIONS_IN_EXECUTION_PAYLOAD_INDEX;

    SIZE + SIZE / FIELDS_COUNT_1 * FIELD_INDEX_1
};

pub const TRANSACTION_IN_BLOCK_BODY: usize = {
    const DEPTH: u32 =
        specs::TRANSACTIONS_DEPTH + 1 + specs::EXECUTION_PAYLOAD_DEPTH + specs::BLOCK_BODY_DEPTH;
    const SIZE: usize = 2usize.pow(DEPTH);

    const FIELDS_COUNT_1: usize = specs::EXECUTION_PAYLOAD_FIELDS_COUNT.next_power_of_two();
    const FIELD_INDEX_1: usize = specs::TRANSACTIONS_IN_EXECUTION_PAYLOAD_INDEX;
    const FIELDS_COUNT_2: usize = specs::BLOCK_BODY_FIELDS_COUNT.next_power_of_two();
    const FIELD_INDEX_2: usize = specs::EXECUTION_PAYLOAD_IN_BLOCK_BODY_INDEX;

    SIZE + SIZE / FIELDS_COUNT_2 * FIELD_INDEX_2
        + SIZE / FIELDS_COUNT_2 / FIELDS_COUNT_1 * FIELD_INDEX_1
};

pub const RECEIPTS_ROOT_IN_EXECUTION_PAYLOAD: usize = {
    const DEPTH: u32 = specs::EXECUTION_PAYLOAD_DEPTH;
    const SIZE: usize = 2usize.pow(DEPTH);

    const FIELDS_COUNT_1: usize = specs::EXECUTION_PAYLOAD_FIELDS_COUNT.next_power_of_two();
    const FIELD_INDEX_1: usize = specs::RECEIPTS_ROOT_IN_EXECUTION_PAYLOAD_INDEX;

    SIZE + SIZE / FIELDS_COUNT_1 * FIELD_INDEX_1
};

pub const RECEIPTS_ROOT_IN_BLOCK_BODY: usize = {
    const DEPTH: u32 = specs::EXECUTION_PAYLOAD_DEPTH + specs::BLOCK_BODY_DEPTH;
    const SIZE: usize = 2usize.pow(DEPTH);

    const FIELDS_COUNT_1: usize = specs::EXECUTION_PAYLOAD_FIELDS_COUNT.next_power_of_two();
    const FIELD_INDEX_1: usize = specs::RECEIPTS_ROOT_IN_EXECUTION_PAYLOAD_INDEX;
    const FIELDS_COUNT_2: usize = specs::BLOCK_BODY_FIELDS_COUNT.next_power_of_two();
    const FIELD_INDEX_2: usize = specs::EXECUTION_PAYLOAD_IN_BLOCK_BODY_INDEX;

    SIZE + SIZE / FIELDS_COUNT_2 * FIELD_INDEX_2
        + SIZE / FIELDS_COUNT_2 / FIELDS_COUNT_1 * FIELD_INDEX_1
};
