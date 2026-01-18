use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use sqlx::{PgPool};
use tokio::time::{sleep, Duration};
use rand::{rng, Rng};

use sqlx::postgres::{PgConnectOptions};




pub async fn connect_to_db(
    database_settings: PgConnectOptions,
) -> Result<PgPool, String> {
    let mut retries = 0;
    let mut delay = Duration::from_millis(1000);

    let max_retries = 5;

    loop {
        match
        PgPool::connect_with(database_settings.clone())
            .await
        {
            Ok(pool) => {
                println!("Connected to Postgres.");
                return Ok(pool);

            }
            Err(err) => {
                retries += 1;
                if max_retries > 0 {
                    if retries > max_retries {
                        panic!("Failed to connect to Postgres after {} retries.", max_retries);
                    }
                }

                // Add jitter to avoid all services retrying at the same time
                let jitter: u64 = rng().random_range(0..1000);
                println!(
                    "Database connection failed (attempt {retries}): {err:?}. Retrying in {:?}...",
                    delay
                );
                sleep(delay + Duration::from_millis(jitter)).await;

                // Exponential backoff, cap at 30s
                delay = std::cmp::min(delay * 2, Duration::from_secs(10));
            }
        }
    }
}

pub fn spawn_db_monitor(pool: PgPool, is_db_up: Arc<AtomicBool>) {
    tokio::spawn(async move {
        let mut last_state: Option<bool> = None;
        let max_number_of_retries = 5;
        let mut retry_number = 0;
        loop {

            let is_up = match sqlx::query("SELECT 1")
                .execute(&pool)
                .await {
                Ok(_) => {
                    //tracing::info!("DB connection successfully established");
                    true
                }
                Err(_) => {
                    if retry_number == max_number_of_retries {
                        //tracing::info!("DB connection could not be successfully established");
                        false
                    }
                    else {
                        retry_number = retry_number + 1;
                        tokio::time::sleep(Duration::from_millis(500)).await;
                        continue;
                    }
                }
            };

            is_db_up.store(is_up, Ordering::Relaxed);

            match (last_state, is_up) {
                (Some(false), true) => tracing::info!("Database is back online"),
                (Some(true), false) => tracing::error!("Database became unreachable"),
                (None, true) => tracing::info!("Database is up"),
                (None, false) => tracing::error!("Database is unreachable"),
                _ => {}
            }

            last_state = Some(is_up);
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });
}