use crate::doctor_fetcher::fetch_doctor_profile;
use crate::models::{DoctorOverview, DoctorProfile};
use futures::{stream, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;

const CONCURRENT_REQUESTS: usize = 10;

pub async fn process_doctors(client: &Client, doctors: Vec<DoctorOverview>) -> Vec<DoctorProfile> {
    let progress_bar = create_progress_bar(doctors.len());

    let doctor_profiles = fetch_doctor_profiles(client, doctors, progress_bar.clone()).await;

    progress_bar.finish_with_message("Processing complete");

    doctor_profiles
}

fn create_progress_bar(total_doctors: usize) -> ProgressBar {
    let pb = ProgressBar::new(total_doctors as u64);
    pb.set_style(ProgressStyle::default_bar());
    pb
}

async fn fetch_doctor_profiles(
    client: &Client,
    doctors: Vec<DoctorOverview>,
    progress_bar: ProgressBar,
) -> Vec<DoctorProfile> {
    stream::iter(doctors)
        .map(|doctor| fetch_single_doctor_profile(client, doctor, progress_bar.clone()))
        .buffer_unordered(CONCURRENT_REQUESTS)
        .filter_map(|result| async move { result })
        .collect()
        .await
}

async fn fetch_single_doctor_profile(
    client: &Client,
    doctor: DoctorOverview,
    progress_bar: ProgressBar,
) -> Option<DoctorProfile> {
    match fetch_doctor_profile(client, doctor.rut).await {
        Ok(profile) => {
            progress_bar.inc(1);
            Some(profile)
        }
        Err(e) => {
            log_error(&doctor, &e);
            progress_bar.inc(1);
            None
        }
    }
}

fn log_error(doctor: &DoctorOverview, error: &Box<dyn std::error::Error>) {
    log::error!("The failing doctor is {:?}", doctor);
    log::error!("Error fetching profile for RUT {}: {}", doctor.rut, error);
}
