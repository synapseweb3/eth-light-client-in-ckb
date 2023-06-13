use std::collections::HashMap;

use eth2_types::{
    light_client_finality_update::LightClientFinalityUpdate, BeaconBlockHeader, MainnetEthSpec,
};

use eth_light_client_in_ckb_verification::{
    types::{core, packed, prelude::*},
    utilities::mmr,
};

use crate::LightClientBootstrap;

pub struct DummyLightClient {
    client: core::Client,
    client_sync_committee: core::ClientSyncCommittee,
    store: mmr::lib::util::MemStore<packed::HeaderDigest>,
    headers: HashMap<u64, mmr::HeaderWithCache>,
}

impl DummyLightClient {
    pub fn new(bootstrap: LightClientBootstrap) -> Self {
        let mut headers = HashMap::default();
        let store = mmr::lib::util::MemStore::default();
        let client = {
            let mut mmr = mmr::ClientRootMMR::new(0, &store);
            let header: core::Header = bootstrap.original().header.clone().into();
            let header_with_cache = header.calc_cache();
            let tip_header_root = header_with_cache.root;
            mmr.push(header_with_cache.packed_digest()).unwrap();
            let headers_mmr_root = mmr.get_root().unwrap().unpack();
            mmr.commit().unwrap();
            let bootstrap_slot = bootstrap.slot().into();
            headers.insert(header_with_cache.inner.slot, header_with_cache);
            core::Client {
                id: 0,
                minimal_slot: bootstrap_slot,
                maximal_slot: bootstrap_slot,
                tip_header_root,
                headers_mmr_root,
            }
        };
        let client_sync_committee = bootstrap.build_client_sync_committee();
        Self {
            client,
            client_sync_committee,
            store,
            headers,
        }
    }

    pub fn client_sync_committee(&self) -> &core::ClientSyncCommittee {
        &self.client_sync_committee
    }

    pub fn client(&self) -> &core::Client {
        &self.client
    }

    pub fn beacon_header_at_slot(&self, slot: u64) -> Option<&mmr::HeaderWithCache> {
        self.headers.get(&slot)
    }

    pub fn build_header_mmr_proof(&self, slot: u64) -> core::MmrProof {
        let index = slot - self.client.minimal_slot;
        let position = mmr::lib::leaf_index_to_pos(index);
        let last_index = self.client.maximal_slot - self.client.minimal_slot;
        let mmr_size = mmr::lib::leaf_index_to_mmr_size(last_index);
        let mmr = mmr::ClientRootMMR::new(mmr_size, &self.store);
        mmr.gen_proof(vec![position])
            .unwrap()
            .proof_items()
            .iter()
            .map(|item| item.unpack())
            .collect::<Vec<_>>()
    }

    pub fn apply_finality_update(
        &mut self,
        finality_update: LightClientFinalityUpdate<MainnetEthSpec>,
        headers: Vec<BeaconBlockHeader>,
    ) -> core::ClientUpdate {
        let (client_update_headers, new_headers_mmr_proof) = {
            let mut client_update_headers = Vec::with_capacity(headers.len());
            let mut positions = Vec::with_capacity(headers.len());
            let last_index = self.client.maximal_slot - self.client.minimal_slot;
            let mmr_size = mmr::lib::leaf_index_to_mmr_size(last_index);
            let mut mmr = mmr::ClientRootMMR::new(mmr_size, &self.store);
            for header in headers {
                let header: core::Header = header.into();
                self.client.maximal_slot = header.slot;

                let index = header.slot - self.client.minimal_slot;
                let position = mmr::lib::leaf_index_to_pos(index);

                let header_with_cache = header.calc_cache();
                self.client.tip_header_root = header_with_cache.root;

                self.headers
                    .insert(header_with_cache.inner.slot, header_with_cache.clone());

                mmr.push(header_with_cache.packed_digest()).unwrap();
                positions.push(position);
                client_update_headers.push(header_with_cache.inner);
            }
            self.client.headers_mmr_root = mmr.get_root().unwrap().unpack();
            let headers_mmr_proof_items = mmr
                .gen_proof(positions)
                .unwrap()
                .proof_items()
                .iter()
                .map(Clone::clone)
                .collect::<Vec<_>>();
            mmr.commit().unwrap();
            let headers_mmr_proof = packed::MmrProof::new_builder()
                .set(headers_mmr_proof_items)
                .build();
            (client_update_headers, headers_mmr_proof.unpack())
        };

        {
            let attested_header = finality_update.attested_header.into();
            let finality_branch = finality_update.finality_branch.to_vec();
            let sync_committee_bits = core::SyncCommitteeBits::from_slice(
                finality_update
                    .sync_aggregate
                    .sync_committee_bits
                    .as_slice(),
            );
            let sync_committee_signature = finality_update
                .sync_aggregate
                .sync_committee_signature
                .serialize()
                .into();
            let sync_aggregate = core::SyncAggregate {
                sync_committee_bits,
                sync_committee_signature,
            };
            let signature_slot = finality_update.signature_slot.into();
            core::ClientUpdate {
                attested_header,
                finality_branch,
                sync_aggregate,
                signature_slot,
                new_headers_mmr_proof,
                headers: client_update_headers,
            }
        }
    }
}
