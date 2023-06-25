use std::fs;

use eth2_types::{
    light_client_bootstrap::PatchedLightClientBootstrap,
    light_client_update::PatchedLightClientUpdate, MainnetEthSpec,
};
use eth_light_client_in_ckb_prover::{LightClientBootstrap, LightClientUpdate};
use eth_light_client_in_ckb_verification::{
    consensus_specs::{forks, helpers},
    types::prelude::*,
};

use crate::{find_json_file, setup, types::load_genesis_validators_root};

#[test]
fn mainnet_testcase_altair_to_bellatrix() {
    let param = Parameter {
        bootstrap_slot: 4612096,
        count: 6,
        ..Default::default()
    };
    sync_committee_update(param);
}

#[test]
fn mainnet_testcase_bellatrix_to_capella() {
    let param = Parameter {
        bootstrap_slot: 6184960,
        count: 6,
        ..Default::default()
    };
    sync_committee_update(param);
}

#[derive(Default)]
struct Parameter {
    bootstrap_slot: u64,
    count: usize,
    dump_dir_opt: Option<&'static str>,
}

fn sync_committee_update(param: Parameter) {
    setup();

    let genesis_validators_root = load_genesis_validators_root();

    let bootstrap: LightClientBootstrap = {
        let case_dir = "mainnet/light_client/bootstrap";
        let filename = format!("slot-{:09}.json", param.bootstrap_slot);
        let json_file = find_json_file(case_dir, &filename);
        let json_str = fs::read_to_string(json_file).unwrap();
        let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        let bootstrap: PatchedLightClientBootstrap<MainnetEthSpec> =
            serde_json::from_value(json_value["data"].clone()).unwrap();
        bootstrap.into()
    };
    let bootstrap_period = helpers::compute_sync_committee_period_at_slot(param.bootstrap_slot);

    let mut current_period = bootstrap_period;
    let mut current_sync_committee = bootstrap.build_client_sync_committee().pack();
    let slots_in_one_period =
        forks::phase0::SLOTS_PER_EPOCH * forks::altair::EPOCHS_PER_SYNC_COMMITTEE_PERIOD;

    if let Some(dump_dir) = param.dump_dir_opt {
        let sync_committee_filepath =
            format!("{dump_dir}/sync_committee-{:09}.data", param.bootstrap_slot);
        fs::write(sync_committee_filepath, current_sync_committee.as_slice()).unwrap();
    }

    for i in 0..param.count {
        let update: LightClientUpdate = {
            let case_dir = "mainnet/light_client/update";
            let filename = format!("period-{:06}.json", current_period);
            let json_file = find_json_file(case_dir, &filename);
            let json_str = fs::read_to_string(json_file).unwrap();
            let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
            let update: PatchedLightClientUpdate<MainnetEthSpec> =
                serde_json::from_value(json_value[0]["data"].clone()).unwrap();
            update.into()
        };

        let sync_committee_update = update.build_sync_committee_update();
        let next_sync_committee = update.build_next_client_sync_committee().pack();

        // first slot in current period
        let first_slot = slots_in_one_period * current_period;
        for (client_max_slot, expected) in [
            (first_slot - 1, false),
            (first_slot, true),
            (first_slot + slots_in_one_period - 1, true),
            (first_slot + slots_in_one_period, false),
        ] {
            let result = sync_committee_update.verify_packed_client_sync_committee(
                client_max_slot,
                genesis_validators_root,
                current_sync_committee.as_reader(),
                next_sync_committee.as_reader(),
            );
            assert_eq!(
                result.is_ok(),
                expected,
                "verify next client sync committee expect {expected} but got opposite \
                (loop: {i}, bootstrap-slot: {}, bootstrap-period: {bootstrap_period}, \
                current-period: {current_period}, client-max-slot: {client_max_slot})",
                param.bootstrap_slot
            );
        }

        if let Some(dump_dir) = param.dump_dir_opt {
            let periods_passed = i + 1;
            let packed_sync_committee_update = sync_committee_update.pack();
            let sync_committee_update_filepath = format!(
                "{dump_dir}/sync_committee_update-{:09}_{periods_passed:02}.data",
                param.bootstrap_slot,
            );
            fs::write(
                sync_committee_update_filepath,
                packed_sync_committee_update.as_slice(),
            )
            .unwrap();
            let sync_committee_filepath = format!(
                "{dump_dir}/sync_committee-{:09}_{periods_passed:02}.data",
                param.bootstrap_slot,
            );
            fs::write(sync_committee_filepath, next_sync_committee.as_slice()).unwrap();
        }

        current_period += 1;
        current_sync_committee = next_sync_committee;
    }
}
