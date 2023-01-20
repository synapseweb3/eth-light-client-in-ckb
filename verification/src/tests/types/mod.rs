use std::{fs::read_to_string, path::PathBuf};

use eth2_types::BeaconBlockHeader;

mod proof_update;
mod transaction_verification;

pub(crate) fn load_beacon_block_header_from_json_or_create_default(
    file: PathBuf,
) -> BeaconBlockHeader {
    let slot: u64 = {
        let file_stem = file.file_stem().unwrap().to_str().unwrap();
        let slot_str = file_stem.strip_prefix("block-header-slot-").unwrap();
        slot_str.parse().unwrap()
    };
    let json_str = read_to_string(file).unwrap();
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
