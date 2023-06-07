use std::sync::Arc;

use eth2_types::{
    light_client_bootstrap::{
        LightClientBootstrap as OriginalLightClientBootstrap, PatchedLightClientBootstrap,
    },
    MainnetEthSpec, Slot,
};

use eth_light_client_in_ckb_verification::{
    consensus_specs::helpers,
    types::{core, prelude::*},
    utilities::mmr,
};

#[derive(Clone)]
pub struct LightClientBootstrap {
    original: OriginalLightClientBootstrap<MainnetEthSpec>,
}

impl From<PatchedLightClientBootstrap<MainnetEthSpec>> for LightClientBootstrap {
    fn from(bootstrap: PatchedLightClientBootstrap<MainnetEthSpec>) -> Self {
        Self {
            original: bootstrap.into(),
        }
    }
}

impl LightClientBootstrap {
    pub fn original(&self) -> &OriginalLightClientBootstrap<MainnetEthSpec> {
        &self.original
    }

    pub fn slot(&self) -> Slot {
        self.original.header.slot
    }

    pub fn build_client_bootstrap(&self) -> core::ClientBootstrap {
        let original = self.original();
        let header: core::Header = original.header.clone().into();
        let current_sync_committee_branch = original.current_sync_committee_branch.to_vec();
        core::ClientBootstrap {
            header,
            current_sync_committee_branch,
        }
    }

    pub fn build_client_sync_committee(&self) -> core::ClientSyncCommittee {
        let original = self.original();
        let period = helpers::compute_sync_committee_period_at_slot(self.slot().into());
        let current_sync_committee = Arc::clone(&original.current_sync_committee);
        let pubkeys = current_sync_committee.pubkeys.to_vec();
        let aggregate_pubkey = current_sync_committee.aggregate_pubkey;
        let data = core::SyncCommittee {
            pubkeys,
            aggregate_pubkey,
        };
        core::ClientSyncCommittee { period, data }
    }

    pub fn build_client(&self) -> core::Client {
        let header = &self.original().header;
        let slot: u64 = header.slot.into();
        let header: core::Header = header.clone().into();
        let header_with_cache = header.calc_cache();
        let headers_mmr_root = {
            let store = mmr::lib::util::MemStore::default();
            let mut mmr = mmr::ClientRootMMR::new(0, &store);
            mmr.push(header_with_cache.packed_digest()).unwrap();
            mmr.get_root().unwrap().unpack()
        };
        core::Client {
            id: 0,
            minimal_slot: slot,
            maximal_slot: slot,
            tip_header_root: header_with_cache.root,
            headers_mmr_root,
        }
    }
}
