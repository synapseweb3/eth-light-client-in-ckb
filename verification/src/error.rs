#[repr(i8)]
pub enum ProofUpdateError {
    EmptyUpdates = 1,
    FirstHeaderSlot,
    FirstHeaderParentRoot,
    UncontinuousSlot,
    UnmatchedParentRoot,
    HeadersMmrProof,
    Other,
}

#[repr(i8)]
pub enum TxVerificationError {
    TransactionSszProof = 1,
    ReceiptMptProof,
    ReceiptsRootSszProof,
    Unsynchronized,
    HeaderMmrProof,
    Other,
}
