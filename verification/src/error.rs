pub enum ProofUpdateError {
    EmptyUpdates,
    FirstHeaderSlot,
    FirstHeaderParentRoot,
    UncontinuousSlot,
    UnmatchedParentRoot,
    HeadersMmrProof,
    Other,
}

pub enum TxVerificationError {
    TransactionSszProof,
    ReceiptMptProof,
    ReceiptsRootSszProof,
    Unsynchronized,
    HeaderMmrProof,
    Other,
}
