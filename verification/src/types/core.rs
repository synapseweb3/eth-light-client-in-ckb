//! The essential rust types.
//!
//! [Packed bytes] are not enough for all usage scenarios.
//!
//! This module provides essential rust types.
//!
//! Most of them is composed of [those packed bytes] or can convert between `self` and [those packed bytes].
//!
//! [Packed bytes]: ../packed/index.html
//! [those packed bytes]: ../packed/index.html

#[cfg(feature = "std")]
use alloc::fmt;
use core::convert::{AsRef, From};

use bls::{PublicKeyBytes, SIGNATURE_BYTES_LEN};
use ethereum_types::H512;
use molecule::prelude::*;
use ssz_derive::Encode;
use tree_hash::Hash256;
use tree_hash_derive::TreeHash;

#[cfg(feature = "std")]
use eth2_types::{BeaconBlockHeader, Slot};
#[cfg(feature = "std")]
use faster_hex::hex_string;

//
// Basic Types
//

/// 64 bits unsigned integer.
pub type Uint64 = u64;
/// 256 bits hash.
pub type Hash = Hash256;
/// Dynamic-length bytes.
pub type Bytes = Vec<u8>;

//
// Proofs, Public Keys and Signatures
//

/// SimpleSerialize (SSZ) Merkle Proof.
///
/// See [SimpleSerialize (SSZ) Merkle Proof Formats] for more details.
///
/// [SimpleSerialize (SSZ) Merkle Proof Formats]: https://github.com/ethereum/consensus-specs/blob/v1.1.0/ssz/merkle-proofs.md
pub type SszProof = Vec<Hash>;

/// Merkle-Patricia Trie (MPT) Proof.
///
/// See [EIP-3120: Binary trie structure] for more details.
///
/// [EIP-3120: Binary trie structure]: https://github.com/ethereum/EIPs/blob/master/EIPS/eip-3102.md
pub type MptProof = Vec<Bytes>;

/// Merkle Node of Merkle Mountain Ranges.
///
/// Ref: [`MmrProof`]
#[derive(Clone)]
pub struct HeaderDigest {
    /// The hash for leaves; otherwise, the hash of children nodes.
    pub children_hash: Hash,
}

/// Merkle Mountain Ranges (MMR) Proof.
///
/// See [Merkle Mountain Ranges] for more details.
///
/// [Merkle Mountain Ranges]: https://github.com/opentimestamps/opentimestamps-server/blob/master/doc/merkle-mountain-range.md
pub type MmrProof = Vec<HeaderDigest>;

/// [BLS Public Key]: G1 points on the BLS12-381 curve.
///
/// [BLS Public Key]: https://github.com/ethereum/consensus-specs/blob/v1.0.0/specs/phase0/validator.md#bls-public-key
pub type BlsPubkey = PublicKeyBytes;
/// A BLS12-381 signature.
///
/// See [Custom types] for more details.
///
/// [Custom types]: https://github.com/ethereum/consensus-specs/blob/v1.0.0/specs/phase0/beacon-chain.md#custom-types
#[derive(Clone, Copy)]
pub struct BlsSignature([u8; SIGNATURE_BYTES_LEN]);
/// A vector of [`BlsPubkey`]s; the size should be [`SYNC_COMMITTEE_SIZE`].
///
/// [`SYNC_COMMITTEE_SIZE`]: ../../consensus_specs/altair/constant.SYNC_COMMITTEE_SIZE.html
pub type BlsPubkeyVec = Vec<BlsPubkey>;

//
// Composite Types
//

/// [The Beacon Block Header.](https://github.com/ethereum/consensus-specs/blob/v1.0.0/specs/phase0/beacon-chain.md#beaconblockheader)
#[derive(Clone, Encode, TreeHash)]
pub struct Header {
    pub slot: Uint64,
    pub proposer_index: Uint64,
    pub parent_root: Hash,
    pub state_root: Hash,
    pub body_root: Hash,
}

/// A dynamic-size vector of [`Header`]s.
pub type HeaderVec = Vec<Header>;

/// [`SYNC_COMMITTEE_SIZE`] bits.
///
/// [`SYNC_COMMITTEE_SIZE`]: ../../consensus_specs/altair/constant.SYNC_COMMITTEE_SIZE.html
pub type SyncCommitteeBits = H512;

/// See [`SyncAggregate`](https://github.com/ethereum/consensus-specs/blob/v1.1.0/specs/altair/beacon-chain.md#syncaggregate).
#[derive(Clone)]
pub struct SyncAggregate {
    pub sync_committee_bits: SyncCommitteeBits,
    pub sync_committee_signature: BlsSignature,
}

/// See [`SyncCommittee`](https://github.com/ethereum/consensus-specs/blob/v1.1.0/specs/altair/beacon-chain.md#synccommittee).
#[derive(Clone)]
pub struct SyncCommittee {
    pub pubkeys: BlsPubkeyVec,
    pub aggregate_pubkey: BlsPubkey,
}

//
// Witnesses
//

/// The data which is used to create all cells.
///
/// References:
/// - [`LightClientBootstrap`](https://github.com/ethereum/consensus-specs/blob/v1.2.0/specs/altair/light-client/sync-protocol.md#lightclientbootstrap)
#[derive(Clone)]
pub struct ClientBootstrap {
    pub header: Header,
    pub current_sync_committee_branch: SszProof,
}

/// The data which is used to update the client cell.
#[derive(Clone)]
pub struct ClientUpdate {
    pub attested_header: Header,
    pub finality_branch: SszProof,
    pub sync_aggregate: SyncAggregate,
    pub signature_slot: Uint64,
    pub new_headers_mmr_proof: MmrProof,
    pub headers: HeaderVec,
}

/// The data which is used to update the sync committee cell.
#[derive(Clone)]
pub struct SyncCommitteeUpdate {
    pub attested_header: Header,
    pub next_sync_committee_branch: SszProof,
    pub sync_aggregate: SyncAggregate,
    pub signature_slot: Uint64,
}

/// A proof which proves a transaction and its receipt are existed in the Ethereum.
#[derive(Clone)]
pub struct TransactionProof {
    /// The header that contains the transaction which requires verification.
    pub header: Header,
    /// The index of the transaction which requires verification.
    pub transaction_index: Uint64,
    /// The `receipts_root` that contains the receipt of the transaction.
    pub receipts_root: Hash,
    /// Prove `header` in `header_mmr_root`.
    pub header_mmr_proof: MmrProof,
    /// Prove `transaction` in `body_root`.
    pub transaction_ssz_proof: SszProof,
    /// Prove `receipt` in `receipts_root`.
    pub receipt_mpt_proof: MptProof,
    /// Prove `receipts_root` in `body_root`.
    pub receipts_root_ssz_proof: SszProof,
}

/// The payload of a transaction and its receipt.
#[derive(Clone)]
pub struct TransactionPayload {
    /// Raw data of the transaction.
    pub transaction: Bytes,
    /// Raw data of the transaction receipt.
    pub receipt: Bytes,
}

//
// Cells
//

/// The client info cell.
#[derive(Clone)]
pub struct ClientInfo {
    /// The ID of the latest light client cell.
    pub last_client_id: u8,
    /// The minimal limit of the updates count.
    pub minimal_headers_count: u8,
    pub genesis_validators_root: Hash,
}

/// The client cell.
#[derive(Clone)]
pub struct Client {
    /// An unique ID of the light client cell.
    pub id: u8,
    /// The minimal slot of the headers in MMR.
    pub minimal_slot: Uint64,
    /// The maximal slot of the headers in MMR.
    pub maximal_slot: Uint64,
    /// The root of the latest finalized header.
    pub tip_header_root: Hash,
    /// The MMR root of headers between slot `minimal_slot` and slot `maximal_slot`.
    pub headers_mmr_root: HeaderDigest,
}

/// The sync committee cell.
#[derive(Clone)]
pub struct ClientSyncCommittee {
    pub period: Uint64,
    pub data: SyncCommittee,
}

/// The args for the type script of client info cell, client sync committee cell and client cells.
#[derive(Clone)]
pub struct ClientTypeArgs {
    pub type_id: Hash,
    /// How many client cells that use current type script.
    ///
    /// N.B. Exclude the client info cell and sync committee cells.
    pub clients_count: u8,
}

#[cfg(feature = "std")]
impl fmt::LowerHex for BlsSignature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{}", hex_string(&self.0))
    }
}

#[cfg(feature = "std")]
impl fmt::Display for BlsSignature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(&self, f)
    }
}

#[cfg(feature = "std")]
impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "{{ slot: {}, empty: true }}", self.slot)
        } else if f.alternate() {
            write!(
                f,
                "{{ slot: {}, parent: {:#x}, state: {:#x}, body: {:#x} }}",
                self.slot, self.parent_root, self.state_root, self.body_root
            )
        } else {
            write!(
                f,
                "{{ slot: {}, parent: {:#x} }}",
                self.slot, self.parent_root
            )
        }
    }
}

#[cfg(feature = "std")]
impl fmt::Display for SyncAggregate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ bits: {:#x}, signature: {} }}",
            self.sync_committee_bits, self.sync_committee_signature
        )
    }
}

#[cfg(feature = "std")]
impl fmt::Display for SyncCommittee {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ pubkeys: [")?;
        if !self.pubkeys.is_empty() {
            write!(f, "{}", &self.pubkeys[0])?;
            for pubkey in &self.pubkeys[1..] {
                write!(f, ", {}", pubkey)?;
            }
        }
        write!(f, "], aggregate: {} }}", self.aggregate_pubkey)
    }
}

#[cfg(feature = "std")]
impl fmt::Display for ClientInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{{ last_client_id: {}, minimal_headers_count: {}, genesis_validators_root: {:#x} }}",
                self.last_client_id, self.minimal_headers_count,self.genesis_validators_root
            )
        } else {
            write!(
                f,
                "{{ last_client_id: {}, minimal_headers_count: {} }}",
                self.last_client_id, self.minimal_headers_count
            )
        }
    }
}

#[cfg(feature = "std")]
impl fmt::Display for ClientSyncCommittee {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ period: {}, data: {} }}", self.period, self.data)
    }
}

#[cfg(feature = "std")]
impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ id: {}, slots: [{}, {}], tip: {:#x} }}",
            self.id, self.minimal_slot, self.maximal_slot, self.tip_header_root
        )
    }
}

impl From<[u8; SIGNATURE_BYTES_LEN]> for BlsSignature {
    fn from(data: [u8; SIGNATURE_BYTES_LEN]) -> Self {
        Self(data)
    }
}

impl From<BlsSignature> for [u8; SIGNATURE_BYTES_LEN] {
    fn from(data: BlsSignature) -> Self {
        data.0
    }
}

impl AsRef<[u8]> for BlsSignature {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(feature = "std")]
impl From<BeaconBlockHeader> for Header {
    fn from(header: BeaconBlockHeader) -> Self {
        Self {
            slot: header.slot.into(),
            proposer_index: header.proposer_index,
            parent_root: header.parent_root,
            state_root: header.state_root,
            body_root: header.body_root,
        }
    }
}

#[cfg(feature = "std")]
impl From<Header> for BeaconBlockHeader {
    fn from(header: Header) -> Self {
        Self {
            slot: Slot::new(header.slot),
            proposer_index: header.proposer_index,
            parent_root: header.parent_root,
            state_root: header.state_root,
            body_root: header.body_root,
        }
    }
}

impl Header {
    /// Checks if a header is empty.
    pub fn is_empty(&self) -> bool {
        self.proposer_index == 0
            && self.parent_root.is_zero()
            && self.state_root.is_zero()
            && self.body_root.is_zero()
    }
}
