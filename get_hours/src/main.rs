use reqwest::{Client, header};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() {
    // Initialize the database connection
    let pool = get_database_pool().await;

    // Get a random doctor ppn (professional ID)
    if let Some(doctor_ppn) = get_random_doctor_ppn(&pool).await {
        // Initialize the HTTP client
        let client = Client::builder()
            .cookie_store(true) // Enable cookie store if needed
            .build()
            .unwrap();

        // Fetch doctor hours
        match fetch_doctor_hours(&client, doctor_ppn).await {
            Ok(html_response) => {
                // For now, just print the response
                println!("HTML Response for doctor ID {}:\n{}", doctor_ppn, html_response);
            }
            Err(e) => {
                eprintln!("Error fetching hours for doctor ID {}: {}", doctor_ppn, e);
            }
        }
    } else {
        eprintln!("No doctor ID found in the database.");
    }
}

async fn get_database_pool() -> SqlitePool {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:../get_doctors/doctors.db".to_string());

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}

async fn get_random_doctor_ppn(pool: &SqlitePool) -> Option<i64> {
    let result = sqlx::query!("SELECT ppn FROM doctors WHERE ppn IS NOT NULL ORDER BY RANDOM() LIMIT 1;")
        .fetch_one(pool)
        .await;

    match result {
        Ok(record) => Some(record.ppn),
        Err(e) => {
            eprintln!("Error fetching doctor ppn: {}", e);
            None
        }
    }
}

async fn fetch_doctor_hours(client: &Client, doctor_id: i64) -> Result<String, reqwest::Error> {
    let url = "https://reserva.alemana.cl/reserva/portal/resultado/profesional";

    let mut params = HashMap::new();
    params.insert("empresa", "1");
    params.insert("tipo_busqueda", "2");
    params.insert("area_medica", "0");
    params.insert("area_interes", "0");
    params.insert("profesional", &doctor_id.to_string());
    params.insert("ubicacion", "99");
    params.insert("fecha", "22/09/2024");
    params.insert("super_centro", "0");
    params.insert("centro", "0");
    params.insert("horario", "A");

    let mut headers = header::HeaderMap::new();

    let response = client
        .get(url)
        .query(&params)
        .headers(headers)
        .send()
        .await?;

    if response.status().is_success() {
        let text = response.text().await?;
        Ok(text)
    } else {
        println!("Error fetching doctor hours: {}", response.status());
    }
}
