use std::fs;

use eth2_types::{
    light_client_bootstrap::PatchedLightClientBootstrap,
    light_client_finality_update::PatchedLightClientFinalityUpdate, MainnetEthSpec,
};
use eth_light_client_in_ckb_prover::DummyLightClient;
use eth_light_client_in_ckb_verification::{consensus_specs::helpers, types::prelude::*};

use crate::{
    find_json_file, setup,
    types::{load_beacon_block_header_from_json_or_create_default, load_genesis_validators_root},
};

#[test]
fn mainnet_testcase_in_capella() {
    let param = Parameter {
        bootstrap_slot: 6632736,
        finalized_slots: vec![6632768, 6632800, 6632832, 6632864, 6632896, 6632928],
        clients_count: 3,
        ..Default::default()
    };
    client_update(param);
}

#[derive(Default)]
struct Parameter {
    bootstrap_slot: u64,
    finalized_slots: Vec<u64>,
    clients_count: u8,
    dump_dir_opt: Option<&'static str>,
}

fn client_update(param: Parameter) {
    setup();

    let genesis_validators_root = load_genesis_validators_root();

    let mut light_client = {
        let case_dir = "mainnet/light_client/bootstrap";
        let filename = format!("slot-{:09}.json", param.bootstrap_slot);
        let json_file = find_json_file(case_dir, &filename);
        let json_str = fs::read_to_string(json_file).unwrap();
        let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        let bootstrap: PatchedLightClientBootstrap<MainnetEthSpec> =
            serde_json::from_value(json_value["data"].clone()).unwrap();
        DummyLightClient::new(bootstrap.into())
    };
    let bootstrap_period = helpers::compute_sync_committee_period_at_slot(param.bootstrap_slot);
    let packed_client_sync_committee = light_client.client_sync_committee().pack();

    let mut client = light_client.client().clone();
    client.id += 1;
    if client.id >= param.clients_count {
        client.id = 0;
    }

    for finalized_slot in param.finalized_slots {
        let finalized_period = helpers::compute_sync_committee_period_at_slot(finalized_slot);
        assert_eq!(
            bootstrap_period, finalized_period,
            "for this test, finalized headers should be \
            in same sync committee period with the bootstrap header, \
            bootstrap {{ slot: {}, period: {bootstrap_period} }} \
            current {{ slot: {finalized_slot}, period: {finalized_period} }}",
            param.bootstrap_slot,
        );

        let finality_update = {
            let case_dir = "mainnet/light_client/finality_update";
            let filename = format!("slot-{:09}.json", finalized_slot);
            let json_file = find_json_file(case_dir, &filename);
            let json_str = fs::read_to_string(json_file).unwrap();
            let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
            let finality_update: PatchedLightClientFinalityUpdate<MainnetEthSpec> =
                serde_json::from_value(json_value["data"].clone()).unwrap();
            finality_update
        };

        let headers = ((client.maximal_slot + 1)..=finalized_slot)
            .map(load_beacon_block_header_from_json_or_create_default)
            .collect::<Vec<_>>();

        let client_update = light_client.apply_finality_update(finality_update.into(), headers);
        let mut new_client = light_client.client().clone();
        new_client.id = client.id;

        let result = client_update.verify_client_update(
            client.clone(),
            genesis_validators_root,
            packed_client_sync_committee.as_reader(),
            new_client.clone(),
        );
        assert!(result.is_ok());

        if let Some(dump_dir) = param.dump_dir_opt {
            let packed_client_update = client_update.pack();
            let client_update_filepath =
                format!("{dump_dir}/client_update-{finalized_slot:09}.data");
            fs::write(client_update_filepath, packed_client_update.as_slice()).unwrap();
            let packed_client = new_client.pack();
            let client_filepath = format!(
                "{dump_dir}/client-{:09}_{finalized_slot:09}.data",
                param.bootstrap_slot
            );
            fs::write(client_filepath, packed_client.as_slice()).unwrap();
        }

        client = new_client;

        client.id += 1;
        if client.id >= param.clients_count {
            client.id = 0;
        }
    }
}
