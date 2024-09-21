mod structs;

use futures::{stream, StreamExt};
use reqwest::Client;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use tokio;

use structs::{Doctor, ResultEntry};

async fn fetch_data(client: &Client, id: u32) -> Result<Vec<Doctor>, Box<dyn Error>> {
    let url = format!(
        "https://portal-backend.clinicaalemana.cl/profs-med/get-by-area?id={}",
        id
    );

    let response = client.get(&url).send().await?;
    let data: Vec<Doctor> = response.json().await?;
    Ok(data)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let ids: Vec<u32> = (1..=200).collect();

    let results: Vec<ResultEntry> = stream::iter(ids)
        .map(|id| {
            let client = &client;
            async move {
                match fetch_data(client, id).await {
                    Ok(doctors) => {
                        println!(
                            "Successfully fetched data for ID {}: {} doctors",
                            id,
                            doctors.len()
                        );
                        Some(ResultEntry { id, doctors })
                    }
                    Err(e) => {
                        eprintln!("Error fetching data for ID {}: {}", id, e);
                        None
                    }
                }
            }
        })
        .buffer_unordered(10)
        .filter_map(|result| async move { result })
        .collect()
        .await;

    println!("Completed {} requests", results.len());

    let json = serde_json::to_string_pretty(&results)?;
    let mut file = File::create("results.json")?;
    file.write_all(json.as_bytes())?;

    println!("Results saved to results.json");

    let total_doctors: usize = results.iter().map(|r| r.doctors.len()).sum();
    println!("Total number of doctors: {}", total_doctors);

    let areas_with_doctors = results.iter().filter(|r| !r.doctors.is_empty()).count();
    println!("Number of areas with doctors: {}", areas_with_doctors);

    Ok(())
}
