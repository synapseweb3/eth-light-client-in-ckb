use alloc::vec::Vec;
use std::fs::read_to_string;

use eth2_types::BeaconBlockHeader;
use tree_hash::TreeHash as _;

use crate::{
    mmr,
    tests::{find_json_files, test_data},
    types::{core, packed, prelude::*},
};

#[test]
fn test_proof_update() {
    let header_json_files = find_json_files("mainnet/beacon", "block-header-slot-");

    let split_at = test_data::COUNT / 2;
    let (headers_part1, headers_part2) = {
        let mut headers = header_json_files
            .into_iter()
            .map(|file| {
                let json_str = read_to_string(file).unwrap();
                let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
                serde_json::from_value(json_value["data"]["header"]["message"].clone()).unwrap()
            })
            .collect::<Vec<BeaconBlockHeader>>();
        let headers_part2 = headers.split_off(split_at);
        (headers, headers_part2)
    };

    let store = mmr::lib::util::MemStore::default();
    let mmr = {
        let mut mmr = mmr::ClientRootMMR::new(0, &store);
        for header in &headers_part1 {
            let header: core::Header = packed::Header::from_ssz_header(header).unpack();
            mmr.push(header.calc_cache().digest()).unwrap();
        }
        mmr
    };

    let last_header_part1 = &headers_part1[headers_part1.len() - 1];
    let client = core::Client {
        minimal_slot: headers_part1[0].slot.into(),
        maximal_slot: last_header_part1.slot.into(),
        tip_header_root: last_header_part1.tree_hash_root(),
        headers_mmr_root: mmr.get_root().unwrap().unpack(),
    };

    let (mmr, packed_headers, new_headers_mmr_proof) = {
        let mut mmr = mmr;
        let mut positions = Vec::with_capacity(headers_part2.len());
        let mut packed_headers = Vec::with_capacity(headers_part2.len());
        for header in &headers_part2 {
            let header_slot: u64 = header.slot.into();
            let index = header_slot - client.minimal_slot;
            let position = mmr::lib::leaf_index_to_pos(index);
            positions.push(position);

            let packed_header = packed::Header::from_ssz_header(header);
            let header = packed_header.unpack();
            let header_with_cache = header.calc_cache();
            mmr.push(header_with_cache.digest()).unwrap();

            packed_headers.push(packed_header);
        }
        let new_headers_mmr_proof_items = mmr
            .gen_proof(positions)
            .unwrap()
            .proof_items()
            .iter()
            .map(Clone::clone)
            .collect::<Vec<_>>();
        let new_headers_mmr_proof = packed::MmrProof::new_builder()
            .set(new_headers_mmr_proof_items)
            .build();
        (mmr, packed_headers, new_headers_mmr_proof)
    };

    let new_headers_mmr_root = mmr.get_root().unwrap();
    let updates_items = packed_headers
        .into_iter()
        .map(|header| {
            packed::FinalityUpdate::new_builder()
                .finalized_header(header)
                .build()
        })
        .collect::<Vec<_>>();
    let updates = packed::FinalityUpdateVec::new_builder()
        .set(updates_items)
        .build();

    let packed_proof_update = packed::ProofUpdate::new_builder()
        .new_headers_mmr_root(new_headers_mmr_root)
        .new_headers_mmr_proof(new_headers_mmr_proof)
        .updates(updates)
        .build();

    let result = client.try_apply_packed_proof_update(packed_proof_update.as_reader());
    assert!(result.is_ok(), "failed to update the proof in client");
}
