mod database;
mod fetch_hours;
mod html_parser;

use reqwest::Client;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = database::get_database_pool().await;

    if let Some(doctor_ppn) = database::get_random_doctor_ppn(&pool).await {
        let client = Client::builder().cookie_store(true).build().unwrap();

        match fetch_hours::fetch_doctor_hours(&client, doctor_ppn).await {
            Ok(html_response) => match html_parser::parse_resultado_parametros(&html_response) {
                Ok(json_data) => {
                    println!("Parsed JSON Data for doctor ID {}", doctor_ppn);
                    let file_name = format!("resultado_parametros_{}.json", doctor_ppn);
                    let mut file = File::create(&file_name).unwrap();
                    file.write_all(json_data.to_string().as_bytes()).unwrap();
                    println!("Saved JSON Data to file {}", file_name);
                }
                Err(e) => {
                    eprintln!("Error parsing HTML response: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Error fetching hours for doctor ID {}: {}", doctor_ppn, e);
            }
        }
    } else {
        eprintln!("No doctor ID found in the database.");
    }

    Ok(())
}
