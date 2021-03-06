use anyhow::Result;
use log::{error, info, warn};
use std::collections::HashMap;
use std::thread;

use geonft_shared::io::{self, SyncStatus};

mod solana;

fn main() -> Result<()> {
    env_logger::init();

    loop {
        let plan = make_plan()?;
        execute_plan(plan)?;
        wait_for_next_round();
    }
}

struct Plan {
    statuses: HashMap<String, io::SyncStatus>,
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

    let statuses = io::get_all_sync_statuses()?;

    let treasure_events = io::get_all_plants_and_claims_time_sorted()?;

    let mut steps = Vec::new();

    for (event, treasure) in treasure_events {
        let pubkey = treasure.public_key;
        let status = statuses.get(&pubkey);

        use io::PlantClaim::{Claim, Plant};
        use io::SyncStatus::*;
        use Step::*;

        match (event, status) {
            (Plant, None) => {
                steps.push((pubkey.clone(), UploadBlobToIpfs));
                steps.push((pubkey, UploadPlantToSolana));
            }
            (Plant, Some(BlobSynced)) => {
                steps.push((pubkey, UploadPlantToSolana));
            }
            (Plant, Some(PlantSynced | ClaimSynced)) => { /* plant is synced */ }
            (Claim, None) | (Claim, Some(BlobSynced | PlantSynced)) => {
                steps.push((pubkey, UploadClaimToSolana));
            }
            (Claim, Some(ClaimSynced)) => { /* claim is synced */ }
        }
    }

    Ok(Plan { statuses, steps })
}

fn execute_plan(plan: Plan) -> Result<()> {
    info!("executing plan with {} steps", plan.steps.len());

    let config = solana::load_config()?;
    let client = solana::connect(&config)?;
    let program_keypair = solana::get_program_keypair(&client)?;
    let program_instance_account =
        solana::get_program_instance_account(&client, &config.keypair, &program_keypair)?;

    let mut statuses = plan.statuses;

    for (pubkey, step) in plan.steps {
        info!("executing step {:?} for {}", step, pubkey);

        let r = || -> Result<()> {
            let status = statuses.get(&pubkey).cloned();
            match step {
                Step::UploadBlobToIpfs => {
                    if status == None {
                        // todo
                        io::record_sync_status(&pubkey, SyncStatus::BlobSynced)?;
                        statuses.insert(pubkey, SyncStatus::BlobSynced);
                    } else {
                        warn!("unexpected sync status: {:?}", status);
                    }
                }
                Step::UploadPlantToSolana => {
                    if status == Some(SyncStatus::BlobSynced) {
                        solana::upload_plant(
                            &pubkey,
                            &config,
                            &client,
                            &program_keypair,
                            &program_instance_account,
                        )?;
                        io::record_sync_status(&pubkey, SyncStatus::PlantSynced)?;
                        statuses.insert(pubkey, SyncStatus::PlantSynced);
                    } else {
                        warn!("unexpected sync status: {:?}", status);
                    }
                }
                Step::UploadClaimToSolana => {
                    if status == Some(SyncStatus::PlantSynced) {
                        solana::upload_claim(
                            &pubkey,
                            &config,
                            &client,
                            &program_keypair,
                            &program_instance_account,
                        )?;
                        io::record_sync_status(&pubkey, SyncStatus::ClaimSynced)?;
                        statuses.insert(pubkey, SyncStatus::ClaimSynced);
                    } else {
                        warn!("unexpected sync status: {:?}", status);
                    }
                }
            }

            Ok(())
        }();

        if let Err(e) = r {
            error!("{}", e);
        } else {
            // info!("successfully executed step {:?} for {}", step, pubkey);
            info!("successfully executed step {:?}", step);
        }
    }

    Ok(())
}

fn wait_for_next_round() {
    let delay_ms = 1000;
    info!("sleeping for {} ms", delay_ms);
    #[allow(deprecated)]
    thread::sleep_ms(delay_ms);
}
