
use deadpool_redis::redis::{AsyncCommands, RedisResult};

use geohash::{encode, Coord};
use crate::org::unibl::etf::model::errors::cache_service_error::CacheServiceError;
use crate::org::unibl::etf::model::requests::store_current_weather_data_request::StoreCurrentWeatherDataRequest;



#[derive(Debug)]
pub struct CurrentWeatherRepository {}




impl CurrentWeatherRepository {
    pub fn new() -> Self {
        Self {}
    }

    #[tracing::instrument(name = "Get Current Weather Cached Data Repository", skip(redis_pool))]
    pub async fn get_current_weather_cache_result(
        &self,
        lat: f64,
        lon: f64,
        redis_pool: &deadpool_redis::Pool,
    ) -> Result<Option<String>, CacheServiceError> {
        let mut conn = match redis_pool.get().await {
            Ok(c) => c,
            Err(e) => {
                let error_message = format!("Failed to get connection from pool: {}", e);
                return Err(CacheServiceError::ServerError(Some(error_message)));
            }
        };
        let coord = Coord { x: lon, y: lat };
        let hash = encode(coord, 5).expect("Invalid coordinates");
        let cache_key = format!("weather:current:{}", hash);

        // 2. Attempt to get from Redis
        let cached_data: Option<String> = conn.get(&cache_key)
            .await
            .map_err(|e| {
                CacheServiceError::RedisError(Some(e.code().unwrap_or("").to_string()), Some(e.to_string()))
            },
        )?;

        Ok(cached_data)

    }

    #[tracing::instrument(name = "Store Current Weather Cache Data Repository", skip(redis_pool))]
    pub async fn store_current_weather_result_as_cache(
        &self,
        data: &StoreCurrentWeatherDataRequest,
        redis_pool: &deadpool_redis::Pool,
    )-> Result<bool, CacheServiceError> {
        let mut conn = match redis_pool.get().await {
            Ok(c) => c,
            Err(e) => {
                let error_message = format!("Failed to get connection from pool: {}", e);
                return Err(CacheServiceError::ServerError(Some(error_message)));
            }
        };

        let coord = Coord { x: data.lon, y: data.lat }; //da nikad ne bude none!!!
        let hash = encode(coord, 5).expect("Invalid coordinates"); //kao env
        let cache_key = format!("weather:current:{}", hash);
        let json = serde_json::to_string(&data.current_weather_data).unwrap();

        let result: RedisResult<()> = conn.set_ex(cache_key, json, 1800).await;


        match result {
            Ok(_) => {
                tracing::info!("Successfully stored current weather data to redis store.");
                Ok(true)
            },
            Err(e) => {
                tracing::error!("Failed to store current weather data to redis store.");
                return Err(CacheServiceError::StoringCacheError(Some(e.to_string())));
            }
        }
    }
}

impl Default for CurrentWeatherRepository {
    fn default() -> Self {
        Self::new()
    }
}