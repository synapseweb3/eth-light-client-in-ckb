use alloc::{format, vec::Vec};
use std::fs;

use eth2_types::BeaconBlockHeader;

use super::load_beacon_block_header_from_json_or_create_default;
use crate::{
    mmr,
    tests::{find_json_files, setup},
    types::{core, packed, prelude::*},
};

const CASE_2_EMPTY_HEADER_INDEX: usize = 47;
const CASE_3_CONTINUOUS_EMPTY_HEADERS_INDEX: usize = 29;

#[test]
fn new_client_case_1_no_empty() {
    let param = NewClientParameter {
        case_id: 1,
        ..Default::default()
    };
    new_client(param);
}

#[test]
#[should_panic(expected = "failed to create client from proof update")]
fn new_client_case_2_empty_at_the_start_of_updates() {
    let skipped_count = CASE_2_EMPTY_HEADER_INDEX;
    let param = NewClientParameter {
        case_id: 2,
        skipped_count_opt: Some(skipped_count),
        ..Default::default()
    };
    new_client(param);
}

#[test]
fn new_client_case_2_empty_at_the_middle_of_updates() {
    let param = NewClientParameter {
        case_id: 2,
        ..Default::default()
    };
    new_client(param);
}

#[test]
fn new_client_case_2_empty_at_the_end_of_updates() {
    let total_count = CASE_2_EMPTY_HEADER_INDEX + 1;
    let param = NewClientParameter {
        case_id: 2,
        total_count_opt: Some(total_count),
        ..Default::default()
    };
    new_client(param);
}

#[test]
#[should_panic(expected = "failed to create client from proof update")]
fn new_client_case_3_continuous_empty_at_the_start_of_updates() {
    let skipped_count = CASE_3_CONTINUOUS_EMPTY_HEADERS_INDEX;
    let param = NewClientParameter {
        case_id: 3,
        skipped_count_opt: Some(skipped_count),
        ..Default::default()
    };
    new_client(param);
}

#[test]
fn new_client_case_3_continuous_empty_at_the_middle_of_updates() {
    let param = NewClientParameter {
        case_id: 3,
        ..Default::default()
    };
    new_client(param);
}

#[test]
fn new_client_case_3_continuous_empty_at_the_end_of_updates() {
    let total_count = CASE_3_CONTINUOUS_EMPTY_HEADERS_INDEX + 2;
    let param = NewClientParameter {
        case_id: 3,
        total_count_opt: Some(total_count),
        ..Default::default()
    };
    new_client(param);
}

#[test]
fn new_client_case_4_during_fork_capella() {
    let param = NewClientParameter {
        case_id: 4,
        ..Default::default()
    };
    new_client(param);
}

#[test]
fn new_client_case_5_after_fork_capella() {
    let param = NewClientParameter {
        case_id: 5,
        ..Default::default()
    };
    new_client(param);
}

#[test]
fn proof_update_case_1_no_empty() {
    let param = ProofUpdateParameter {
        case_id: 1,
        ..Default::default()
    };
    proof_update(param);
}

#[test]
fn proof_update_case_2_empty_client() {
    let split_at = CASE_2_EMPTY_HEADER_INDEX + 1;
    let param = ProofUpdateParameter {
        case_id: 2,
        split_at_opt: Some(split_at),
        ..Default::default()
    };
    proof_update(param);
}

#[test]
fn proof_update_case_2_empty_at_the_start_of_updates() {
    let split_at = CASE_2_EMPTY_HEADER_INDEX;
    let param = ProofUpdateParameter {
        case_id: 2,
        split_at_opt: Some(split_at),
        ..Default::default()
    };
    proof_update(param);
}

#[test]
fn proof_update_case_2_empty_at_the_middle_of_updates() {
    let param = ProofUpdateParameter {
        case_id: 2,
        ..Default::default()
    };
    proof_update(param);
}

#[test]
fn proof_update_case_2_empty_at_the_end_of_updates() {
    let total_count = CASE_2_EMPTY_HEADER_INDEX + 1;
    let param = ProofUpdateParameter {
        case_id: 2,
        total_count_opt: Some(total_count),
        ..Default::default()
    };
    proof_update(param);
}

#[test]
fn proof_update_case_3_continuous_empty_client() {
    let split_at = CASE_3_CONTINUOUS_EMPTY_HEADERS_INDEX + 2;
    let param = ProofUpdateParameter {
        case_id: 3,
        split_at_opt: Some(split_at),
        ..Default::default()
    };
    proof_update(param);
}

#[test]
fn proof_update_case_3_continuous_empty_at_the_start_of_updates() {
    let split_at = CASE_3_CONTINUOUS_EMPTY_HEADERS_INDEX;
    let param = ProofUpdateParameter {
        case_id: 3,
        split_at_opt: Some(split_at),
        ..Default::default()
    };
    proof_update(param);
}

#[test]
fn proof_update_case_3_continuous_empty_at_the_middle_of_updates() {
    let param = ProofUpdateParameter {
        case_id: 3,
        ..Default::default()
    };
    proof_update(param);
}

#[test]
fn proof_update_case_3_continuous_empty_at_the_middle_of_updates_and_split() {
    let split_at = CASE_3_CONTINUOUS_EMPTY_HEADERS_INDEX + 1;
    let param = ProofUpdateParameter {
        case_id: 3,
        split_at_opt: Some(split_at),
        ..Default::default()
    };
    proof_update(param);
}

#[test]
fn proof_update_case_3_continuous_empty_at_the_end_of_updates() {
    let total_count = CASE_3_CONTINUOUS_EMPTY_HEADERS_INDEX + 2;
    let param = ProofUpdateParameter {
        case_id: 3,
        total_count_opt: Some(total_count),
        ..Default::default()
    };
    proof_update(param);
}

#[test]
fn proof_update_case_4_during_fork_capella() {
    let param = ProofUpdateParameter {
        case_id: 4,
        ..Default::default()
    };
    proof_update(param);
}

#[test]
fn proof_update_case_5_after_fork_capella() {
    let param = ProofUpdateParameter {
        case_id: 5,
        ..Default::default()
    };
    proof_update(param);
}

#[derive(Default)]
struct NewClientParameter {
    case_id: usize,
    skipped_count_opt: Option<usize>,
    total_count_opt: Option<usize>,
    dump_dir_opt: Option<&'static str>,
}

fn new_client(param: NewClientParameter) {
    setup();

    let case_dir = format!("mainnet/case-{}/beacon", param.case_id);

    let headers = {
        let mut header_json_files = find_json_files(&case_dir, "block-header-slot-");
        if let Some(total_count) = param.total_count_opt {
            header_json_files.truncate(total_count);
        }
        let skipped_count = param.skipped_count_opt.unwrap_or(0);
        header_json_files
            .into_iter()
            .skip(skipped_count)
            .map(load_beacon_block_header_from_json_or_create_default)
            .collect::<Vec<BeaconBlockHeader>>()
    };

    let minimal_slot: u64 = headers[0].slot.into();
    let maximal_slot: u64 = headers[headers.len() - 1].slot.into();

    let (tip_valid_header_root, packed_headers, headers_mmr_root, headers_mmr_proof) = {
        let store = mmr::lib::util::MemStore::default();
        let mut mmr = mmr::ClientRootMMR::new(0, &store);
        let mut positions = Vec::with_capacity(headers.len());
        let mut packed_headers = Vec::with_capacity(headers.len());
        let mut tip_valid_header_root_opt = None;

        for header in &headers {
            let header_slot: u64 = header.slot.into();
            let index = header_slot - minimal_slot;
            let position = mmr::lib::leaf_index_to_pos(index);

            let packed_header = packed::Header::from_ssz_header(header);
            let header: core::Header = packed_header.unpack();
            let header_with_cache = header.calc_cache();

            if !header_with_cache.inner.is_empty() {
                tip_valid_header_root_opt = Some(header_with_cache.root);
            }

            mmr.push(header_with_cache.digest()).unwrap();
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

        (
            tip_valid_header_root_opt.unwrap(),
            packed_headers,
            headers_mmr_root,
            headers_mmr_proof,
        )
    };

    let expected_packed_client = core::Client {
        minimal_slot,
        maximal_slot,
        tip_valid_header_root,
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

    if let Some(ref dump_dir) = param.dump_dir_opt {
        let client_filepath = format!("{dump_dir}/client-{minimal_slot}_{maximal_slot}.data");
        let proof_update_filepath =
            format!("{dump_dir}/proof_update-{minimal_slot}_{maximal_slot}.data");
        fs::write(client_filepath, expected_packed_client.as_slice()).unwrap();
        fs::write(proof_update_filepath, packed_proof_update.as_slice()).unwrap();
    }

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

#[derive(Default)]
struct ProofUpdateParameter {
    case_id: usize,
    total_count_opt: Option<usize>,
    split_at_opt: Option<usize>,
    dump_dir_opt: Option<&'static str>,
}

fn proof_update(param: ProofUpdateParameter) {
    setup();

    let case_dir = format!("mainnet/case-{}/beacon", param.case_id);

    let (headers_part1, headers_part2) = {
        let mut header_json_files = find_json_files(&case_dir, "block-header-slot-");
        if let Some(total_count) = param.total_count_opt {
            header_json_files.truncate(total_count);
        }
        let split_at = param
            .split_at_opt
            .unwrap_or_else(|| header_json_files.len() / 2);
        let mut headers = header_json_files
            .into_iter()
            .map(load_beacon_block_header_from_json_or_create_default)
            .collect::<Vec<BeaconBlockHeader>>();
        let headers_part2 = headers.split_off(split_at);
        (headers, headers_part2)
    };

    let store = mmr::lib::util::MemStore::default();
    let (tip_valid_header_root, headers_mmr_root, mmr) = {
        let mut mmr = mmr::ClientRootMMR::new(0, &store);
        let mut tip_valid_header_root_opt = None;
        for header in &headers_part1 {
            let header: core::Header = packed::Header::from_ssz_header(header).unpack();
            let header_with_cache = header.calc_cache();
            if !header_with_cache.inner.is_empty() {
                tip_valid_header_root_opt = Some(header_with_cache.root);
            }
            mmr.push(header_with_cache.digest()).unwrap();
        }
        let headers_mmr_root = mmr.get_root().unwrap().unpack();
        (tip_valid_header_root_opt.unwrap(), headers_mmr_root, mmr)
    };

    let minimal_slot = headers_part1[0].slot.into();
    let maximal_slot = headers_part1[headers_part1.len() - 1].slot.into();
    let client = core::Client {
        minimal_slot,
        maximal_slot,
        tip_valid_header_root,
        headers_mmr_root,
    };

    let (new_tip_valid_header_root, packed_headers, new_headers_mmr_root, new_headers_mmr_proof) = {
        let mut mmr = mmr;
        let mut positions = Vec::with_capacity(headers_part2.len());
        let mut packed_headers = Vec::with_capacity(headers_part2.len());
        let mut tip_valid_header_root_opt = None;

        for header in &headers_part2 {
            let header_slot: u64 = header.slot.into();
            let index = header_slot - client.minimal_slot;
            let position = mmr::lib::leaf_index_to_pos(index);

            let packed_header = packed::Header::from_ssz_header(header);
            let header = packed_header.unpack();
            let header_with_cache = header.calc_cache();

            if !header_with_cache.inner.is_empty() {
                tip_valid_header_root_opt = Some(header_with_cache.root);
            }

            mmr.push(header_with_cache.digest()).unwrap();
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

        (
            tip_valid_header_root_opt.unwrap(),
            packed_headers,
            headers_mmr_root,
            headers_mmr_proof,
        )
    };

    let new_minimal_slot: u64 = headers_part2[0].slot.into();
    assert_eq!(new_minimal_slot, maximal_slot + 1);
    let new_maximal_slot = headers_part2[headers_part2.len() - 1].slot.into();
    let expected_packed_client = core::Client {
        minimal_slot,
        maximal_slot: new_maximal_slot,
        tip_valid_header_root: new_tip_valid_header_root,
        headers_mmr_root: new_headers_mmr_root.unpack(),
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
        .new_headers_mmr_root(new_headers_mmr_root)
        .new_headers_mmr_proof(new_headers_mmr_proof)
        .updates(updates)
        .build();

    if let Some(ref dump_dir) = param.dump_dir_opt {
        let client_filepath = format!("{dump_dir}/client-{minimal_slot}_{maximal_slot}.data");
        let new_client_filepath =
            format!("{dump_dir}/client-{minimal_slot}_{new_maximal_slot}.data");
        let proof_update_filepath =
            format!("{dump_dir}/proof_update-{new_minimal_slot}_{new_maximal_slot}.data");
        let packed_client = client.pack();
        fs::write(client_filepath, packed_client.as_slice()).unwrap();
        fs::write(new_client_filepath, expected_packed_client.as_slice()).unwrap();
        fs::write(proof_update_filepath, packed_proof_update.as_slice()).unwrap();
    }

    let result = client.try_apply_packed_proof_update(packed_proof_update.as_reader());
    assert!(result.is_ok(), "failed to update the proof in client");

    if let Ok(actual_client) = result {
        let actual_packed_client = actual_client.pack();
        assert_eq!(
            actual_packed_client.as_slice(),
            expected_packed_client.as_slice()
        );
    }
}
