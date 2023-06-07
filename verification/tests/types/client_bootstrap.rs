use std::fs::read_to_string;

use eth2_types::{light_client_bootstrap::PatchedLightClientBootstrap, MainnetEthSpec};
use eth_light_client_in_ckb_prover::LightClientBootstrap;
use eth_light_client_in_ckb_verification::types::prelude::*;

use crate::{find_json_files, setup};

#[test]
fn test_client_bootstraps() {
    setup();

    let case_dir = "mainnet/light_client/bootstrap";
    let json_files = find_json_files(case_dir, "slot-");

    let bootstraps = json_files
        .into_iter()
        .map(|file| {
            let json_str = read_to_string(file).unwrap();
            let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
            let bootstrap: PatchedLightClientBootstrap<MainnetEthSpec> =
                serde_json::from_value(json_value["data"].clone()).unwrap();
            bootstrap.into()
        })
        .collect::<Vec<LightClientBootstrap>>();

    for bootstrap in bootstraps {
        let slot = bootstrap.slot();

        let client_bootstrap = bootstrap.build_client_bootstrap();
        let packed_client_sync_committee = bootstrap.build_client_sync_committee().pack();
        let reader = packed_client_sync_committee.as_reader();
        let result = client_bootstrap.verify_packed_client_sync_committee(reader);
        assert!(
            result.is_ok(),
            "failed to verify client sync committee (slot: {slot})"
        );

        let expected_packed_client = bootstrap.build_client().pack();
        let packed_client = client_bootstrap.header.initialize_client().pack();
        assert_eq!(
            expected_packed_client.as_slice(),
            packed_client.as_slice(),
            "failed to verify client (slot: {slot})"
        );
    }
}
