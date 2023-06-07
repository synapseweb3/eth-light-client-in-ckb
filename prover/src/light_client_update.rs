use std::sync::Arc;

use eth2_types::{
    light_client_update::{
        LightClientUpdate as OriginalLightClientUpdate, PatchedLightClientUpdate,
    },
    MainnetEthSpec,
};

use eth_light_client_in_ckb_verification::{consensus_specs::helpers, types::core};

#[derive(Clone)]
pub struct LightClientUpdate {
    original: OriginalLightClientUpdate<MainnetEthSpec>,
}

impl From<PatchedLightClientUpdate<MainnetEthSpec>> for LightClientUpdate {
    fn from(update: PatchedLightClientUpdate<MainnetEthSpec>) -> Self {
        Self {
            original: update.into(),
        }
    }
}

impl LightClientUpdate {
    pub fn original(&self) -> &OriginalLightClientUpdate<MainnetEthSpec> {
        &self.original
    }

    pub fn build_sync_committee_update(&self) -> core::SyncCommitteeUpdate {
        let original = self.original();
        let attested_header = original.attested_header.clone().into();
        let next_sync_committee_branch = original.next_sync_committee_branch.to_vec();
        let sync_committee_bits = core::SyncCommitteeBits::from_slice(
            original.sync_aggregate.sync_committee_bits.as_slice(),
        );
        let sync_committee_signature = original
            .sync_aggregate
            .sync_committee_signature
            .serialize()
            .into();
        let sync_aggregate = core::SyncAggregate {
            sync_committee_bits,
            sync_committee_signature,
        };
        let signature_slot = original.signature_slot.into();
        core::SyncCommitteeUpdate {
            attested_header,
            next_sync_committee_branch,
            sync_aggregate,
            signature_slot,
        }
    }

    pub fn build_next_client_sync_committee(&self) -> core::ClientSyncCommittee {
        let attested_slot = self.original.attested_header.slot;
        let attested_period = helpers::compute_sync_committee_period_at_slot(attested_slot.into());
        let period = attested_period + 1;
        let next_sync_committee = Arc::clone(&self.original().next_sync_committee);
        let pubkeys = next_sync_committee.pubkeys.to_vec();
        let aggregate_pubkey = next_sync_committee.aggregate_pubkey;
        let data = core::SyncCommittee {
            pubkeys,
            aggregate_pubkey,
        };
        core::ClientSyncCommittee { period, data }
    }
}
