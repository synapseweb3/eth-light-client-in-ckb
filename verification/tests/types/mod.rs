use std::fs;

use eth2_types::BeaconBlockHeader;
use tree_hash::Hash256;

use crate::find_json_file;

mod client_bootstrap;
mod client_update;
mod sync_committee_update;
mod transaction_verification;

pub(crate) fn load_beacon_block_header_from_json_or_create_default(slot: u64) -> BeaconBlockHeader {
    let case_dir = "mainnet/beacon/header";
    let filename = format!("slot-{:09}.json", slot);
    let json_file = find_json_file(case_dir, &filename);
    let json_str = fs::read_to_string(json_file).unwrap();
    let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    if json_value.get("code").is_some() {
        BeaconBlockHeader {
            slot: slot.into(),
            proposer_index: 0,
            parent_root: Default::default(),
            state_root: Default::default(),
            body_root: Default::default(),
        }
    } else {
        serde_json::from_value(json_value["data"]["header"]["message"].clone()).unwrap()
    }
}

pub(crate) fn load_genesis_validators_root() -> Hash256 {
    let json_file = find_json_file("mainnet", "beacon_genesis.json");
    let json_str = fs::read_to_string(json_file).unwrap();
    let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
    serde_json::from_value(json_value["genesis_validators_root"].clone()).unwrap()
}

#[test]
fn mainnet_genesis_validators_root() {
    let dump_dir_opt = None;
    mainnet_genesis_validators_root_internal(dump_dir_opt);
}

fn mainnet_genesis_validators_root_internal(dump_dir_opt: Option<&'static str>) {
    let genesis_validators_root = load_genesis_validators_root();
    if let Some(dump_dir) = dump_dir_opt {
        let filepath = format!("{dump_dir}/genesis_validators_root.data");
        fs::write(filepath, genesis_validators_root.as_ref()).unwrap();
    }
}
