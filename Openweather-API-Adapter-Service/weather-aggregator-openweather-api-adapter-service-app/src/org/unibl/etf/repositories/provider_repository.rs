
use deadpool_redis::redis::{pipe, AsyncCommands, RedisResult};
use crate::org::unibl::etf::model::errors::adapter_service_error::AdapterServiceError;

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
    ) -> Result<i64, AdapterServiceError> {
        let mut conn = match redis_pool.get().await {
            Ok(c) => c,
            Err(e) => {
                let error_message = format!("Failed to get connection from pool: {}", e);
                return Err(AdapterServiceError::ServerError(Some(error_message)));
            }
        };

        let key = format!("provider:{}:ratelimit", provider_name);

        let number_of_requests: Option<i64> = conn.get(&key)
            .await
            .map_err(|e| {
                AdapterServiceError::RedisError(
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
    )-> Result<i64, AdapterServiceError> {
        let mut conn = match redis_pool.get().await {
            Ok(c) => c,
            Err(e) => {
                let error_message = format!("Failed to get connection from pool: {}", e);
                return Err(AdapterServiceError::ServerError(Some(error_message)));
            }
        };

        let key = format!("provider:{}:ratelimit", provider_name);

        let result: RedisResult<(i64, i64)> = pipe()
            //.atomic()
            .incr(&key, 1)
            .expire(&key, 1800)
            .query_async(&mut conn)
            .await;

        let current_count = match result {
            Ok((current_count, _expire_successful)) => {
                tracing::info!("Successfully incremented number of requests in redis store.");
                current_count
            },
            Err(e) => {
                tracing::error!("Failed to increment number of requests in redis store.");
                return Err(AdapterServiceError::RedisError(
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