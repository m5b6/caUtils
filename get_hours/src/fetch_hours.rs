use chrono;
use reqwest::{header, Client};
use std::{collections::HashMap, error::Error};

#[allow(non_snake_case)]
pub async fn fetch_doctor_hours(client: &Client, doctor_id: i64) -> Result<String, Box<dyn Error>> {
    let url = "https://reserva.alemana.cl/reserva/portal/resultado/profesional";
    let doctor_id = doctor_id.to_string();
    let firstDayOfCurrentMonth = chrono::Local::now().date_naive();
    let formattedFirstDayOfCurrentMonth = firstDayOfCurrentMonth.format("%d/%m/%Y").to_string();

    let mut params = HashMap::new();
    params.insert("empresa", "1");
    params.insert("tipo_busqueda", "2");
    params.insert("area_medica", "0");
    params.insert("area_interes", "0");
    params.insert("profesional", &doctor_id);
    params.insert("ubicacion", "99");
    params.insert("fecha", &formattedFirstDayOfCurrentMonth);
    params.insert("super_centro", "0");
    params.insert("centro", "0");
    params.insert("horario", "A");

    let headers = header::HeaderMap::new();

    let response = client
        .get(url)
        .query(&params)
        .headers(headers)
        .send()
        .await?;

    if response.status() == reqwest::StatusCode::OK {
        let text = response.text().await?;
        Ok(text)
    } else if response.status() == reqwest::StatusCode::UNAUTHORIZED {
        println!("Error fetching doctor hours: {}", response.status());
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "Unauthorized",
        )))
    } else {
        println!(
            "Error fetching doctor hours: {}",
            response.status().as_u16()
        );
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unknown error",
        )))
    }
}
