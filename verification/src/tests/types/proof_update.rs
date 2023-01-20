use alloc::{format, vec::Vec};

use eth2_types::BeaconBlockHeader;
use tree_hash::TreeHash as _;

use super::load_beacon_block_header_from_json_or_create_default;
use crate::{
    mmr,
    tests::find_json_files,
    types::{core, packed, prelude::*},
};

#[test]
fn test_new_client_case_1() {
    test_new_client(1);
}

#[test]
fn test_new_client_case_2() {
    test_new_client(2);
}

#[test]
fn test_proof_update_case_1() {
    test_proof_update(1);
}

#[test]
fn test_proof_update_case_2() {
    test_proof_update(2);
}

fn test_new_client(case_id: usize) {
    let case_dir = format!("mainnet/case-{}/beacon", case_id);

    let header_json_files = find_json_files(&case_dir, "block-header-slot-");

    let headers = header_json_files
        .into_iter()
        .map(load_beacon_block_header_from_json_or_create_default)
        .collect::<Vec<BeaconBlockHeader>>();

    let last_header = &headers[headers.len() - 1];
    let minimal_slot: u64 = headers[0].slot.into();
    let maximal_slot: u64 = last_header.slot.into();
    let tip_header_root = last_header.tree_hash_root();

    let (packed_headers, headers_mmr_root, headers_mmr_proof) = {
        let store = mmr::lib::util::MemStore::default();
        let mut mmr = mmr::ClientRootMMR::new(0, &store);
        let mut positions = Vec::with_capacity(headers.len());
        let mut packed_headers = Vec::with_capacity(headers.len());

        for header in &headers {
            let header_slot: u64 = header.slot.into();
            let index = header_slot - minimal_slot;
            let position = mmr::lib::leaf_index_to_pos(index);

            let packed_header = packed::Header::from_ssz_header(header);
            let header: core::Header = packed_header.unpack();

            mmr.push(header.calc_cache().digest()).unwrap();
            positions.push(position);
            packed_headers.push(packed_header);
        }

        let headers_mmr_root = mmr.get_root().unwrap();
        let headers_mmr_proof_items = mmr
            .gen_proof(positions)
            .unwrap()
            .proof_items()
            .iter()
            .map(Clone::clone)
            .collect::<Vec<_>>();
        let headers_mmr_proof = packed::MmrProof::new_builder()
            .set(headers_mmr_proof_items)
            .build();

        (packed_headers, headers_mmr_root, headers_mmr_proof)
    };

    let expected_packed_client = core::Client {
        minimal_slot,
        maximal_slot,
        tip_header_root,
        headers_mmr_root: headers_mmr_root.unpack(),
    }
    .pack();

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
        .new_headers_mmr_root(headers_mmr_root)
        .new_headers_mmr_proof(headers_mmr_proof)
        .updates(updates)
        .build();

    let result = core::Client::new_from_packed_proof_update(packed_proof_update.as_reader());
    assert!(result.is_ok(), "failed to create client from proof update");

    if let Ok(actual_client) = result {
        let actual_packed_client = actual_client.pack();

        assert_eq!(
            actual_packed_client.as_slice(),
            expected_packed_client.as_slice()
        );
    }
}

fn test_proof_update(case_id: usize) {
    let case_dir = format!("mainnet/case-{}/beacon", case_id);

    let header_json_files = find_json_files(&case_dir, "block-header-slot-");

    let split_at = header_json_files.len() / 2;
    let (headers_part1, headers_part2) = {
        let mut headers = header_json_files
            .into_iter()
            .map(load_beacon_block_header_from_json_or_create_default)
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
