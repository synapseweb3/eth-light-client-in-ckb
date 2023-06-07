#[repr(i8)]
pub enum ClientBootstrapError {
    // Verify Self
    HeaderIsEmpty = 1,
    // Verify Current Sync Committee
    IncorrectPeriod,
    UnexpectedSyncCommitteeSize,
    InvalidSyncCommitteeBranch,
    // This is not an error, just make sure the error code is less than 32.
    Unreachable = 32,
}

#[repr(i8)]
pub enum ClientUpdateError {
    // Verify Self
    AttestedHeaderIsEmpty = 1,
    BadSignatureSlot,
    // Check Headers
    EmptyHeaders,
    FirstHeaderSlot,
    FirstHeaderParentRoot,
    UncontinuousSlot,
    UnmatchedParentRoot,
    HeadersMmrProof,
    FinalizedHeaderIsEmpty,
    // Check Clients
    ClientIdChanged,
    ClientMinimalSlotChanged,
    ClientMaximalSlot,
    ClientTipHeaderRoot,
    // Check Current Sync Committee
    MismatchedSyncCommittee,
    // Verify the Signature with Current Sync Committee
    NotSupermajorityParticipation,
    FailedToVerifyTheAttestedHeader,
    // Verify Finality Header
    FinalizedShouldBeAfterAttested,
    InvalidFinalityBranch,
    // Internal Errors
    MmrError,
    BlsPublicKeyBytesError,
    BlsAggregateSignatureError,
    // This is not an error, just make sure the error code is less than 32.
    Unreachable = 32,
}

#[repr(i8)]
pub enum SyncCommitteeUpdateError {
    // Verify Self
    AttestedHeaderIsEmpty = 1,
    BadSignatureSlot,
    // Check Current Sync Committee
    BadCurrentPeriod,
    SignatureInNextPeriod,
    // Verify the Signature with Current Sync Committee
    NotSupermajorityParticipation,
    FailedToVerifyTheAttestedHeader,
    // Verify Next Sync Committee
    NoncontinuousPeriods,
    UnexpectedNextSyncCommitteeSize,
    InvalidNextSyncCommitteeBranch,
    // Internal Errors
    BlsPublicKeyBytesError,
    BlsAggregateSignatureError,
    // This is not an error, just make sure the error code is less than 32.
    Unreachable = 32,
}

#[repr(i8)]
pub enum TxVerificationError {
    // Verify Header
    Unsynchronized = 1,
    HeaderMmrProof,
    // Verify Transaction
    TransactionSszProof,
    // Verify Receipt
    ReceiptMptProof,
    ReceiptsRootSszProof,
    // Internal Errors
    MmrError,
    SszError,
    // This is not an error, just make sure the error code is less than 32.
    Unreachable = 32,
}
