mod models;
mod utils;

use futures::{stream, StreamExt};
use reqwest::Client;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use tokio;

use models::{Doctor, DoctorProfile, ResultEntry};
use utils::format_rut_with_verification_digit;

async fn fetch_area_data(client: &Client, id: u32) -> Result<Vec<Doctor>, Box<dyn Error>> {
    let url = format!(
        "https://portal-backend.clinicaalemana.cl/profs-med/get-by-area?id={}",
        id
    );
    let response = client.get(&url).send().await?;
    response.json().await.map_err(Into::into)
}

async fn fetch_doctor_profile(client: &Client, rut: u64) -> Result<DoctorProfile, Box<dyn Error>> {
    let formatted_rut = format_rut_with_verification_digit(rut);
    let url = format!(
        "https://portal-backend.clinicaalemana.cl/profs-med/profile?id={}",
        formatted_rut
    );
    println!("Fetching profile for RUT {}\nURL: {}", formatted_rut, url);
    let response = client.get(&url).send().await?;
    response.json().await.map_err(Into::into)
}

async fn process_area(client: &Client, id: u32) -> Option<ResultEntry> {
    match fetch_area_data(client, id).await {
        Ok(doctors) => {
            println!(
                "Successfully fetched data for ID {}: {} doctors",
                id,
                doctors.len()
            );
            let doctor_profiles = fetch_doctor_profiles(client, doctors).await;
            Some(ResultEntry {
                id,
                doctors: doctor_profiles,
            })
        }
        Err(e) => {
            eprintln!("Error fetching data for ID {}: {}", id, e);
            None
        }
    }
}

async fn fetch_doctor_profiles(client: &Client, doctors: Vec<Doctor>) -> Vec<DoctorProfile> {
    stream::iter(doctors)
        .map(|doctor| {
            let rut = doctor.rut.clone();
            async move {
                match fetch_doctor_profile(client, rut).await {
                    Ok(profile) => Some(profile),
                    Err(e) => {
                        eprintln!("Error fetching profile for RUT {}: {}", rut, e);
                        None
                    }
                }
            }
        })
        .buffer_unordered(5)
        .filter_map(|result| async move { result })
        .collect()
        .await
}

async fn process_all_areas(client: &Client, ids: Vec<u32>) -> Vec<ResultEntry> {
    stream::iter(ids)
        .map(|id| process_area(client, id))
        .buffer_unordered(10)
        .filter_map(|result| async move { result })
        .collect()
        .await
}

fn save_results_to_file(results: &[ResultEntry]) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(&results)?;
    let mut file = File::create("results.json")?;
    file.write_all(json.as_bytes())?;
    println!("Results saved to results.json");
    Ok(())
}

fn print_statistics(results: &[ResultEntry]) {
    let total_doctors: usize = results.iter().map(|r| r.doctors.len()).sum();
    println!("Total number of doctors: {}", total_doctors);

    let areas_with_doctors = results.iter().filter(|r| !r.doctors.is_empty()).count();
    println!("Number of areas with doctors: {}", areas_with_doctors);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let ids: Vec<u32> = (1..=200).collect();

    let results = process_all_areas(&client, ids).await;

    println!("Completed {} requests", results.len());

    save_results_to_file(&results)?;
    print_statistics(&results);

    Ok(())
}
