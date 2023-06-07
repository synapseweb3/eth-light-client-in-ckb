use std::fs::read_to_string;

use eth2_types::{
    light_client_bootstrap::PatchedLightClientBootstrap,
    light_client_finality_update::PatchedLightClientFinalityUpdate, BeaconBlock, MainnetEthSpec,
};
use eth_light_client_in_ckb_prover::{CachedBeaconBlock, DummyLightClient, Receipts};
use eth_light_client_in_ckb_verification::types::{core, prelude::*};
use ethers_core::types::TransactionReceipt;

use crate::{find_json_file, setup, types::load_beacon_block_header_from_json_or_create_default};

#[derive(Default)]
struct Parameter {
    bootstrap_slot: u64,
    finalized_slots: Vec<u64>,
    test_blocks: Vec<u64>,
}

#[test]
fn test_case_1() {
    let param = Parameter {
        bootstrap_slot: 6632736,
        finalized_slots: vec![6632768, 6632800, 6632832, 6632864, 6632896, 6632928],
        test_blocks: vec![6632854],
    };
    test_transaction_verification(param);
}

fn test_transaction_verification(param: Parameter) {
    setup();

    let mut light_client = {
        let case_dir = "mainnet/light_client/bootstrap";
        let filename = format!("slot-{:09}.json", param.bootstrap_slot);
        let json_file = find_json_file(case_dir, &filename);
        let json_str = read_to_string(json_file).unwrap();
        let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        let bootstrap: PatchedLightClientBootstrap<MainnetEthSpec> =
            serde_json::from_value(json_value["data"].clone()).unwrap();
        DummyLightClient::new(bootstrap.into())
    };

    for finalized_slot in param.finalized_slots {
        let finality_update = {
            let case_dir = "mainnet/light_client/finality_update";
            let filename = format!("slot-{:09}.json", finalized_slot);
            let json_file = find_json_file(case_dir, &filename);
            let json_str = read_to_string(json_file).unwrap();
            let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
            let finality_update: PatchedLightClientFinalityUpdate<MainnetEthSpec> =
                serde_json::from_value(json_value["data"].clone()).unwrap();
            finality_update
        };
        let headers = ((light_client.client().maximal_slot + 1)..=finalized_slot)
            .map(load_beacon_block_header_from_json_or_create_default)
            .collect::<Vec<_>>();
        let _client_update = light_client.apply_finality_update(finality_update.into(), headers);

        let client = light_client.client().clone();

        for block_slot in &param.test_blocks {
            if *block_slot > finalized_slot {
                continue;
            }

            let block: CachedBeaconBlock = {
                let case_dir = "mainnet/beacon/block";
                let filename = format!("slot-{:09}.json", block_slot);
                let json_file = find_json_file(case_dir, &filename);
                let json_str = read_to_string(json_file).unwrap();
                let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
                let block: BeaconBlock<MainnetEthSpec> =
                    serde_json::from_value(json_value["data"]["message"].clone()).unwrap();
                block.into()
            };

            let slot: u64 = block.slot().into();
            let number = block.number();

            let receipts: Receipts = {
                let case_dir = "mainnet/execution/block_receipts";
                let filename = format!("number-{:09}.json", number);
                let json_file = find_json_file(case_dir, &filename);
                let json_str = read_to_string(json_file).unwrap();
                let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
                let receipts: Vec<TransactionReceipt> =
                    serde_json::from_value(json_value["result"].clone()).unwrap();
                receipts.into()
            };

            let transactions_count = block.transactions_count();
            let receipts_count = receipts.original().len();
            assert_eq!(
                transactions_count, receipts_count,
                "failed to check the receipts size for block#{}: expect {} but actual {}",
                slot, transactions_count, receipts_count,
            );

            let receipts_root = receipts.root();
            let receipts_root_ssz_proof = block.generate_receipts_root_proof_for_block_body();

            let header = light_client.beacon_header_at_slot(slot).unwrap();
            let header_mmr_proof = light_client.build_header_mmr_proof(slot);

            for index in 0..receipts_count {
                let transaction_ssz_proof = block.generate_transaction_proof_for_block_body(index);
                let receipt_mpt_proof = receipts.generate_proof(index);

                let proof = core::TransactionProof {
                    header: header.inner.clone(),
                    transaction_index: index as u64,
                    receipts_root,
                    header_mmr_proof: header_mmr_proof.clone(),
                    transaction_ssz_proof,
                    receipt_mpt_proof,
                    receipts_root_ssz_proof: receipts_root_ssz_proof.clone(),
                };

                let payload = core::TransactionPayload {
                    transaction: block.transaction(index).unwrap().to_vec(),
                    receipt: receipts.encode_data(index),
                };

                let packed_proof = proof.pack();
                let packed_payload = payload.pack();

                let result = client.verify_packed_transaction_proof(packed_proof.as_reader());
                assert!(
                    result.is_ok(),
                    "failed to verify packed proof for block#{}.transaction#{}",
                    number,
                    index
                );

                let result = proof.verify_packed_payload(packed_payload.as_reader());
                assert!(
                    result.is_ok(),
                    "failed to verify packed payload for block#{}.transaction#{}",
                    number,
                    index
                );
            }
        }
    }
}
