// doctor_processor.rs
use crate::doctor_fetcher::fetch_doctor_profile;
use crate::models::{DoctorOverview, DoctorProfile};
use futures::{stream, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

pub async fn process_doctors(client: &Client, doctors: Vec<DoctorOverview>) -> Vec<DoctorProfile> {
    let total_doctors = doctors.len() as u64;

    let pb = ProgressBar::new(total_doctors);
    pb.set_style(ProgressStyle::default_bar());

    let profiles = stream::iter(doctors)
        .map(|doctor| {
            let client = client.clone();
            let pb = pb.clone();
            let rut = doctor.rut;
            async move {
                match fetch_doctor_profile(&client, rut).await {
                    Ok(profile) => {
                        pb.inc(1);
                        Some(profile)
                    }
                    Err(e) => {
                        log::error!("The failing doctor is {:?}", doctor);
                        log::error!("Error fetching profile for RUT {}: {}", rut, e);
                        pb.inc(1);
                        None
                    }
                }
            }
        })
        .buffer_unordered(10)
        .filter_map(|result| async move { result })
        .collect()
        .await;

    pb.finish_with_message("Processing complete");

    profiles
}
