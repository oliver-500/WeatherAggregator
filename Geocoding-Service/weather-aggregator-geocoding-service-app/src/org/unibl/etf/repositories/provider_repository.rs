use std::error::Error;
use deadpool_redis::redis::{pipe, AsyncCommands, RedisResult, Value};
use crate::org::unibl::etf::model::errors::geocoding_service_error::GeocodingServiceError;

#[derive(Debug)]
pub struct ProviderRepository{

}


impl ProviderRepository {
    pub fn new() -> Self {
        Self {}
    }

    #[tracing::instrument(name = "Get Number of requests for a provider repository", skip(redis_pool))]
    pub async fn get_number_of_requests(
        &self,
        provider_name: &str,
        redis_pool: &deadpool_redis::Pool
    ) -> Result<i64, GeocodingServiceError> {
        let mut conn = match redis_pool.get().await {
            Ok(c) => c,
            Err(e) => {
                let error_message = format!("Failed to get connection from pool: {}", e);
                return Err(GeocodingServiceError::ServerError(Some(error_message)));
            }
        };

        let key = format!("provider:{}:ratelimit", provider_name);

        let number_of_requests: Option<i64> = conn.get(&key)
            .await
            .map_err(|e| {
                GeocodingServiceError::RedisError(
                    Some(e.code().unwrap_or("").to_string()), Some(e.to_string())
                )
            })?;

        match number_of_requests {
            Some(number_of_requests) => Ok(number_of_requests),
            None => { Ok(0)}
        }
    }
    #[tracing::instrument(name = "Increment number of requests repository", skip(redis_pool))]
    pub async fn increment_number_of_requests(
        &self,
        provider_name: &str,
        redis_pool: &deadpool_redis::Pool,
    )-> Result<i64, GeocodingServiceError> {
        let mut conn = match redis_pool.get().await {
            Ok(c) => c,
            Err(e) => {
                let error_message = format!("Failed to get connection from pool: {}", e);
                return Err(GeocodingServiceError::ServerError(Some(error_message)));
            }
        };

        let key = format!("provider:{}:ratelimit", provider_name);

        // let (current_count, ) = match pipe()
        //     .atomic()
        //     .incr(&key, 1)
        //     .expire(&key, 20)
        //     .ignore()
        //     .query_async::<(i64,)>(&mut conn)
        //     .await
        // {
        //     Ok(v) => v,
        //     Err(e) => {
        //         tracing::error!(error = ?e, "Redis error (debug)");
        //         if let Some(src) = e.source() {
        //             tracing::error!(source = ?src, "Redis error source");
        //         }
        //         return Err(GeocodingServiceError::RedisError(
        //                      Some(e.code().unwrap_or("").to_string()), Some(e.to_string())
        //                  ));
        //     }
        // };


        let result: RedisResult<(i64, i64)> = pipe()
            //.atomic()
            .incr(&key, 1)
            .expire(&key, 1800)
            .query_async(&mut conn)
            .await;

        //tracing::info!("RAW REDIS RESPONSE: {:?}", raw);

        let current_count = match result {
            Ok((current_count, expire_successful)) => {
                tracing::info!("Successfully incremented number of requests in redis store.");
                current_count
            },
            Err(e) => {
                tracing::error!("Failed to increment number of requests in redis store.");
                return Err(GeocodingServiceError::RedisError(
                    Some(e.code().unwrap_or("").to_string()), Some(e.to_string())
                ));
            }
        };

        Ok(current_count)

    }
}

impl Default for ProviderRepository {
    fn default() -> Self {
        Self::new()
    }
}