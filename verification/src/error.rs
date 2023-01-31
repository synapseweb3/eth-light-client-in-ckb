#[repr(i8)]
pub enum ProofUpdateError {
    EmptyUpdates = 1,
    FirstHeaderForCreate,
    FirstHeaderSlot,
    FirstHeaderParentRoot,
    UncontinuousSlot,
    UnmatchedParentRoot,
    HeadersMmrProof,
    Other = 15,
}

#[repr(i8)]
pub enum TxVerificationError {
    TransactionSszProof = 1,
    ReceiptMptProof,
    ReceiptsRootSszProof,
    Unsynchronized,
    HeaderMmrProof,
    Other = 15,
}
