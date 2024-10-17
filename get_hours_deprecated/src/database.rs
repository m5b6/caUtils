use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Row;
use sqlx::SqlitePool;
use std::env;

pub async fn get_database_pool() -> SqlitePool {
    dotenv::dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:../get_doctors/doctors.db".to_string());

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}

pub async fn get_random_doctor_ppn(pool: &SqlitePool) -> Option<i64> {
    let result =
        sqlx::query("SELECT ppn FROM doctors WHERE ppn IS NOT NULL ORDER BY RANDOM() LIMIT 1;")
            .fetch_one(pool)
            .await;

    match result {
        Ok(record) => {
            let ppn: i64 = record.try_get("ppn").unwrap();
            Some(ppn)
        }
        Err(e) => {
            eprintln!("Error fetching doctor ppn: {}", e);
            None
        }
    }
}
