
use deadpool_redis::redis::{AsyncCommands, RedisResult};

use geohash::{encode, Coord};
use crate::org::unibl::etf::model::errors::cache_service_error::CacheServiceError;

use crate::org::unibl::etf::model::responses::current_weather_cache_response::CurrentWeatherCacheResponse;


#[derive(Debug)]
pub struct CurrentWeatherRepository {}



impl CurrentWeatherRepository {
    pub fn new() -> Self {
        Self {}
    }

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
        let hash = encode(coord, 6).expect("Invalid coordinates");
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

    pub async fn store_current_weather_result_as_cache(
        &self,
        data: &CurrentWeatherCacheResponse,
        redis_pool: &deadpool_redis::Pool,
    )-> Result<bool, CacheServiceError> {
        let mut conn = match redis_pool.get().await {
            Ok(c) => c,
            Err(e) => {
                let error_message = format!("Failed to get connection from pool: {}", e);
                return Err(CacheServiceError::ServerError(Some(error_message)));
            }
        };

        let coord = Coord { x: data.location.lon.unwrap(), y: data.location.lat.unwrap() }; //da nikad ne bude none!!!
        let hash = encode(coord, 6).expect("Invalid coordinates"); //kao env
        let cache_key = format!("weather:current:{}", hash);
        let json = serde_json::to_string(data).unwrap();

        let result: RedisResult<()> = conn.set_ex(cache_key, json, 1800).await;

        match result {
            Ok(_) => Ok(true),
            Err(e) => {
                return Err(CacheServiceError::ConnectionError(Some(e.to_string())));
            }
        }
    }
}

impl Default for CurrentWeatherRepository {
    fn default() -> Self {
        Self::new()
    }
}