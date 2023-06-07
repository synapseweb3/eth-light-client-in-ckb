use std::fs::read_to_string;

use eth2_types::BeaconBlockHeader;
use eth_light_client_in_ckb_verification::types::core;
use tree_hash::Hash256;

use crate::find_json_files;

#[test]
fn test_header_root_case_1() {
    test_header_root(1);
}

#[test]
fn test_header_root_case_2() {
    test_header_root(2);
}

#[test]
fn test_header_root_case_3() {
    test_header_root(3);
}

#[test]
fn test_header_root_case_4() {
    test_header_root(4);
}

#[test]
fn test_header_root_case_5() {
    test_header_root(5);
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
        let header: core::Header = ssz_header.into();
        let header_with_cache = header.calc_cache();
        assert_eq!(
            expected_root, header_with_cache.root,
            "failed to check the root of beacon block header#{}: expect {:#x} but actual {:#x}",
            header_with_cache.inner.slot, expected_root, header_with_cache.root,
        );
    }
}
