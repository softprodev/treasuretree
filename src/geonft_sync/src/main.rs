#![allow(unused)]

use anyhow::Result;
use log::info;
use std::collections::HashMap;
use std::thread;
use std::net::SocketAddr;
use std::time::Duration;

use geonft_shared::data;
use solana_client::thin_client::{self, ThinClient};
use solana_sdk::client::SyncClient;

fn main() -> Result<()> {
    env_logger::init();

    loop {
        let plan = make_plan()?;
        execute_plan(plan)?;
        wait_for_next_round();
    }
}

struct Plan {
    statuses: HashMap<String, data::SyncStatus>,
    steps: Vec<(String, Step)>,
}

#[derive(Debug)]
enum Step {
    UploadBlobToIpfs,
    UploadPlantToSolana,
    UploadClaimToSolana,
}

fn make_plan() -> Result<Plan> {
    info!("making new plan");

    let statuses = data::get_all_sync_statuses()?;

    let treasure_events = data::get_all_plants_and_claims_time_sorted()?;

    let mut steps = Vec::new();

    for (event, treasure) in treasure_events {
        let pubkey = treasure.public_key;
        let status = statuses.get(&pubkey);

        use data::PlantClaim::{Plant, Claim};
        use data::SyncStatus::*;
        use Step::*;

        match (event, status) {
            (Plant, None) => {
                steps.push((pubkey.clone(), UploadBlobToIpfs));
                steps.push((pubkey, UploadPlantToSolana));
            },
            (Plant, Some(BlobSynced)) => {
                steps.push((pubkey, UploadPlantToSolana));
            },
            (Plant, Some(PlantSynced | ClaimSynced)) => {
                /* plant is synced */
            },
            (Claim, None) | (Claim, Some(BlobSynced | PlantSynced)) => {
                steps.push((pubkey, UploadClaimToSolana));
            }
            (Claim, Some(ClaimSynced)) => {
                /* claim is synced */
            }
        }
    }

    Ok(Plan {
        statuses,
        steps
    })
}

fn execute_plan(plan: Plan) -> Result<()> {
    info!("executing plan with {} steps", plan.steps.len());

    let rpc_addr = "127.0.0.1:8899";
    let tpu_addr = "127.0.0.1:1027";
    let tx_port_range = (10_000_u16, 20_000_u16);
    let timeout = 1000;

    info!("connecting to solana node, RPC: {}, TPU: {}, tx range: {}-{}, timeout: {}ms",
          rpc_addr, tpu_addr, tx_port_range.0, tx_port_range.1, timeout);

    let rpc_addr: SocketAddr = rpc_addr.parse().expect("");
    let tpu_addr: SocketAddr = tpu_addr.parse().expect("");

    let client = thin_client::create_client_with_timeout(
        (rpc_addr, tpu_addr),
        tx_port_range,
        Duration::from_millis(timeout));

    let epoch = client.get_epoch_info()?;

    info!("{:?}", epoch);

    let mut statuses = plan.statuses;

    for (pubkey, step) in plan.steps {
        info!("executing step {:?} for {}", step, pubkey);
    }

    Ok(())
}

fn wait_for_next_round() {
    let delay_ms = 1000;
    info!("sleeping for {} ms", delay_ms);
    #[allow(deprecated)]
    thread::sleep_ms(delay_ms);
}
