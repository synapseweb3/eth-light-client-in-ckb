macro_rules! define_generalized_index_mod {
    ($containers:path) => {
        /// Pre-computed parameters for [generalized index].
        ///
        /// [generalized index]: https://github.com/ethereum/consensus-specs/blob/v1.0.0/ssz/merkle-proofs.md#generalized-merkle-tree-index
        pub mod generalized_index {

            use $containers as containers;

            /// Offset to calculate generalized index for a transaction in transactions data.
            pub const TRANSACTION_IN_TRANSACTIONS_DATA_OFFSET: usize =
                2usize.pow(containers::TRANSACTIONS_DEPTH);

            /// Offset to calculate generalized index for a transaction in `Transactions`.
            pub const TRANSACTION_IN_TRANSACTIONS_OFFSET: usize =
                2usize.pow(containers::TRANSACTIONS_DEPTH + 1);

            /// Offset to calculate generalized index for a transaction in `ExecutionPayload`.
            pub const TRANSACTION_IN_EXECUTION_PAYLOAD_OFFSET: usize = {
                const DEPTH: u32 =
                    containers::TRANSACTIONS_DEPTH + 1 + containers::EXECUTION_PAYLOAD_DEPTH;
                const SIZE: usize = 2usize.pow(DEPTH);

                const FIELDS_COUNT_1: usize =
                    containers::EXECUTION_PAYLOAD_FIELDS_COUNT.next_power_of_two();
                const FIELD_INDEX_1: usize = containers::TRANSACTIONS_IN_EXECUTION_PAYLOAD_INDEX;

                SIZE + SIZE / FIELDS_COUNT_1 * FIELD_INDEX_1
            };

            /// Offset to calculate generalized index for a transaction in `BlockBody`.
            pub const TRANSACTION_IN_BLOCK_BODY_OFFSET: usize = {
                const DEPTH: u32 = containers::TRANSACTIONS_DEPTH
                    + 1
                    + containers::EXECUTION_PAYLOAD_DEPTH
                    + containers::BLOCK_BODY_DEPTH;
                const SIZE: usize = 2usize.pow(DEPTH);

                const FIELDS_COUNT_1: usize =
                    containers::EXECUTION_PAYLOAD_FIELDS_COUNT.next_power_of_two();
                const FIELD_INDEX_1: usize = containers::TRANSACTIONS_IN_EXECUTION_PAYLOAD_INDEX;
                const FIELDS_COUNT_2: usize =
                    containers::BLOCK_BODY_FIELDS_COUNT.next_power_of_two();
                const FIELD_INDEX_2: usize = containers::EXECUTION_PAYLOAD_IN_BLOCK_BODY_INDEX;

                SIZE + SIZE / FIELDS_COUNT_2 * FIELD_INDEX_2
                    + SIZE / FIELDS_COUNT_2 / FIELDS_COUNT_1 * FIELD_INDEX_1
            };

            /// Generalized index for `receipts_root` in `ExecutionPayload`.
            pub const RECEIPTS_ROOT_IN_EXECUTION_PAYLOAD: usize = {
                const DEPTH: u32 = containers::EXECUTION_PAYLOAD_DEPTH;
                const SIZE: usize = 2usize.pow(DEPTH);

                const FIELDS_COUNT_1: usize =
                    containers::EXECUTION_PAYLOAD_FIELDS_COUNT.next_power_of_two();
                const FIELD_INDEX_1: usize = containers::RECEIPTS_ROOT_IN_EXECUTION_PAYLOAD_INDEX;

                SIZE + SIZE / FIELDS_COUNT_1 * FIELD_INDEX_1
            };

            /// Generalized index for `receipts_root` in `BlockBody`.
            pub const RECEIPTS_ROOT_IN_BLOCK_BODY: usize = {
                const DEPTH: u32 =
                    containers::EXECUTION_PAYLOAD_DEPTH + containers::BLOCK_BODY_DEPTH;
                const SIZE: usize = 2usize.pow(DEPTH);

                const FIELDS_COUNT_1: usize =
                    containers::EXECUTION_PAYLOAD_FIELDS_COUNT.next_power_of_two();
                const FIELD_INDEX_1: usize = containers::RECEIPTS_ROOT_IN_EXECUTION_PAYLOAD_INDEX;
                const FIELDS_COUNT_2: usize =
                    containers::BLOCK_BODY_FIELDS_COUNT.next_power_of_two();
                const FIELD_INDEX_2: usize = containers::EXECUTION_PAYLOAD_IN_BLOCK_BODY_INDEX;

                SIZE + SIZE / FIELDS_COUNT_2 * FIELD_INDEX_2
                    + SIZE / FIELDS_COUNT_2 / FIELDS_COUNT_1 * FIELD_INDEX_1
            };
        }
    };
}
