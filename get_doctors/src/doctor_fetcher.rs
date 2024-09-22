use reqwest::Client;
use crate::models::DoctorOverview;
use crate::models::DoctorProfile;
use std::error::Error;

pub async fn fetch_all_doctors(client: &Client) -> Result<Vec<DoctorOverview>, Box<dyn Error>> {
    let url = "https://portal-backend.clinicaalemana.cl/profs-med/get-by-area?id=*";
    let response = client.get(url).send().await?;
    let response_json: Vec<DoctorOverview> = response.json().await?;
    Ok(response_json)
}

pub async fn fetch_doctor_profile(client: &Client, rut: u64) -> Result<DoctorProfile, Box<dyn Error>> {
    let formatted_rut = crate::utils::format_rut_with_verification_digit(rut);
    let url = format!(
        "https://portal-backend.clinicaalemana.cl/profs-med/profile?id={}",
        formatted_rut
    );
    let response = client.get(&url).send().await?;
    response.json().await.map_err(Into::into)
}