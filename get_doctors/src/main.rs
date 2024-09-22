mod database;
mod doctor_fetcher;
mod doctor_processor;
mod logger;
mod models;
mod utils;

use colored::*;
use sqlx::SqlitePool;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::init().unwrap();

    let database_url = "sqlite:clinicaalemana.db";
    let pool = SqlitePool::connect(&database_url).await?;

    database::initialize_database(&pool).await?;

    let client = reqwest::Client::new();

    log::info!("{}", "Fetching all doctors...".green().bold());
    let all_doctors = doctor_fetcher::fetch_all_doctors(&client).await?;
    let initial_count = all_doctors.len();
    log::info!(
        "Total number of doctors fetched: {}",
        initial_count.to_string().blue()
    );

    log::info!("{}", "Processing doctor profiles...".green().bold());
    let doctor_profiles = doctor_processor::process_doctors(&client, all_doctors).await;
    let final_count = doctor_profiles.len();

    database::save_results_to_database(&pool, &doctor_profiles).await?;
    utils::print_statistics(initial_count, final_count);

    Ok(())
}
