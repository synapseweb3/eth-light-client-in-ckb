//! The utilities for [BLS12-381 curve (BLS)] for [Validators in the Ethereum 2.0 Protocol].
//!
//! [BLS12-381 curve (BLS)]: https://electriccoin.co/blog/new-snark-curve/
//! [Validators in the Ethereum 2.0 Protocol]: https://github.com/ethereum/consensus-specs/blob/v1.0.0/specs/phase0/validator.md#bls-public-key

use alloc::vec::Vec;

use bls::{AggregateSignature, Error as BlsError, PublicKey, PublicKeyBytes};
use ssz_types::{typenum, FixedVector};
use tree_hash::{Hash256, TreeHash as _};
use tree_hash_derive::TreeHash;

use crate::{consensus_specs::helpers, types::core};

impl core::SyncCommittee {
    /// Decompress the bytes of public keys into actual public keys.
    pub fn decompress_all_pubkeys(&self) -> Result<Vec<PublicKey>, BlsError> {
        self.pubkeys
            .iter()
            .map(PublicKeyBytes::decompress)
            .collect()
    }
}

impl core::SyncAggregate {
    /// Verifies the signature against the given public keys and one message.
    ///
    /// N.B. `pubkeys.len()` should be checked.
    pub fn fast_aggregate_verify(
        &self,
        pubkeys: &[PublicKey],
        message: Hash256,
    ) -> Result<bool, BlsError> {
        let participant_pubkeys = self.filter_participant_pubkeys(pubkeys);
        AggregateSignature::deserialize(self.sync_committee_signature.as_ref())
            .map(|signature| signature.fast_aggregate_verify(message, &participant_pubkeys))
    }

    /// Filters the participant public keys with a bits vector.
    ///
    /// N.B. `pubkeys.len() == self.sync_committee_bits.bits_size()` should be checked.
    pub fn filter_participant_pubkeys<'a>(&self, pubkeys: &'a [PublicKey]) -> Vec<&'a PublicKey> {
        let bits = self.sync_committee_bits.as_ref();
        assert_eq!(pubkeys.len(), bits.len() * 8);
        pubkeys
            .iter()
            .enumerate()
            .filter_map(|(i, pubkey)| {
                let x = i / 8;
                let y = i % 8;
                if bits[x] & (1 << y) == 0 {
                    None
                } else {
                    Some(pubkey)
                }
            })
            .collect()
    }
}

/// Computes the signing root for the corresponding data at the given `signature_slot`.
pub fn compute_signing_root_at_signature_slot(
    signed_root: Hash256,
    signature_slot: u64,
    domain_type: &[u8; 4],
    genesis_validators_root: Hash256,
) -> Hash256 {
    let fork_version_slot = if signature_slot > 0 {
        signature_slot - 1
    } else {
        0
    };
    let fork_version =
        helpers::compute_fork_version(helpers::compute_epoch_at_slot(fork_version_slot));
    let domain = compute_domain(domain_type, &fork_version, genesis_validators_root);
    compute_signing_root(signed_root, domain)
}

/// See [`SigningData`].
///
/// [`SigningData`]: https://github.com/ethereum/consensus-specs/blob/v1.0.0/specs/phase0/beacon-chain.md#signingdata
#[derive(TreeHash)]
pub struct SigningData {
    pub object_root: Hash256,
    pub domain: Hash256,
}

/// Returns the signing root for the corresponding signing data.
///
/// See [`compute_signing_root`].
///
/// [`compute_signing_root`]: https://github.com/ethereum/consensus-specs/blob/v1.0.0/specs/phase0/beacon-chain.md#compute_signing_root
pub fn compute_signing_root(object_root: Hash256, domain: Hash256) -> Hash256 {
    SigningData {
        object_root,
        domain,
    }
    .tree_hash_root()
}

/// Returns the domain for the `domain_type` and `fork_version`.
///
/// See [`compute_domain`].
///
/// [`compute_domain`]: https://github.com/ethereum/consensus-specs/blob/v1.0.0/specs/phase0/beacon-chain.md#compute_domain
pub fn compute_domain(
    domain_type: &[u8; 4],
    fork_version: &[u8; 4],
    genesis_validators_root: Hash256,
) -> Hash256 {
    let fork_data_root = compute_fork_data_root(fork_version, genesis_validators_root);
    let mut domain = [0u8; 32];
    domain[..4].copy_from_slice(domain_type);
    domain[4..].copy_from_slice(&fork_data_root[..28]);
    Hash256::from(domain)
}

/// See [`ForkData`].
///
/// [`ForkData`]: https://github.com/ethereum/consensus-specs/blob/v1.0.0/specs/phase0/beacon-chain.md#forkdata
#[derive(TreeHash)]
pub struct ForkData {
    pub current_version: FixedVector<u8, typenum::U4>,
    pub genesis_validators_root: Hash256,
}

impl ForkData {
    pub fn new(version: &[u8; 4], genesis_validators_root: Hash256) -> Self {
        let current_version = FixedVector::from(version.to_vec());
        Self {
            current_version,
            genesis_validators_root,
        }
    }
}

/// Returns the 32-byte fork data root for the ``current_version`` and ``genesis_validators_root``.
/// This is used primarily in signature domains to avoid collisions across forks/chains.
///
/// See [`compute_fork_data_root`].
///
/// [`compute_fork_data_root`]: https://github.com/ethereum/consensus-specs/blob/v1.0.0/specs/phase0/beacon-chain.md#compute_fork_data_root
pub fn compute_fork_data_root(
    current_version: &[u8; 4],
    genesis_validators_root: Hash256,
) -> Hash256 {
    ForkData::new(current_version, genesis_validators_root).tree_hash_root()
}
