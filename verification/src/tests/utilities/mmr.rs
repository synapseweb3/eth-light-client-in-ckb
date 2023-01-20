use alloc::format;
use std::fs::read_to_string;

use eth2_types::BeaconBlockHeader;
use tree_hash::Hash256;

use crate::{
    tests::find_json_files,
    types::{core, packed, prelude::*},
};

#[test]
fn test_header_root_case_1() {
    test_header_root(1);
}

#[test]
fn test_header_root_case_2() {
    test_header_root(2);
}

fn test_header_root(case_id: usize) {
    let case_dir = format!("mainnet/case-{}/beacon", case_id);

    let json_files = find_json_files(&case_dir, "block-header-slot-");

    for json_file in json_files {
        let json_str = read_to_string(json_file).unwrap();
        let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        if json_value.get("code").is_some() {
            continue;
        }
        let expected_root: Hash256 =
            serde_json::from_value(json_value["data"]["root"].clone()).unwrap();
        let ssz_header: BeaconBlockHeader =
            serde_json::from_value(json_value["data"]["header"]["message"].clone()).unwrap();
        let header: core::Header = packed::Header::from_ssz_header(&ssz_header).unpack();
        let actual_root = header.calc_cache().root;
        assert_eq!(
            expected_root, actual_root,
            "failed to check the root of beacon block header#{}: expect {:#x} but actual {:#x}",
            ssz_header.slot, expected_root, actual_root,
        );
    }
}
