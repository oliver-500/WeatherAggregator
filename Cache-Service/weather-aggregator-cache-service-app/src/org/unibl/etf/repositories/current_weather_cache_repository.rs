
use deadpool_redis::redis::{AsyncCommands, RedisResult};

use geohash::{encode, Coord};
use crate::org::unibl::etf::model::errors::cache_service_error::CacheServiceError;
use crate::org::unibl::etf::model::requests::retrieve_current_weather_cache_request::RetrieveCurrentWeatherCacheRequest;
use crate::org::unibl::etf::model::requests::store_current_weather_data_request::StoreCurrentWeatherDataRequest;



#[derive(Debug)]
pub struct CurrentWeatherRepository {}




impl CurrentWeatherRepository {
    pub fn new() -> Self {
        Self {}
    }

    #[tracing::instrument(name = "Get Current Weather Cached Data Repository", skip(redis_pool))]
    pub async fn retrieve_current_weather_cache_result_by_coordinates(
        &self,
        req: &RetrieveCurrentWeatherCacheRequest,
        redis_pool: &deadpool_redis::Pool,
    ) -> Result<Option<String>, CacheServiceError> {
        let mut conn = match redis_pool.get().await {
            Ok(c) => c,
            Err(e) => {
                let error_message = format!("Failed to get connection from pool: {}", e);
                return Err(CacheServiceError::ServerError(Some(error_message)));
            }
        };

        if !req.lat.is_some() || !req.lon.is_some() {
            return Err(CacheServiceError::ServerError(Some("Latitude and longitude not provided".to_string())));
        }
        let coord = Coord { x: req.lon.unwrap(), y: req.lat.unwrap() };
        let hash = encode(coord, 5).expect("Invalid coordinates");
        let cache_key = format!("weather:current:{}", hash);
        let result: Option<String> = conn.get(&cache_key)
            .await
            .map_err(|e| {
                CacheServiceError::RedisError(Some(e.code().unwrap_or("").to_string()), Some(e.to_string()))
            },
            )?;

        Ok(result)
    }

    #[tracing::instrument(name = "Get Current Weather Cached Data Repository", skip(redis_pool))]
    pub async fn retrieve_current_weather_cache_result_by_location(
        &self,
        req: &RetrieveCurrentWeatherCacheRequest,
        redis_pool: &deadpool_redis::Pool,
    ) -> Result<Vec<String>, CacheServiceError> {
        let mut conn = match redis_pool.get().await {
            Ok(c) => c,
            Err(e) => {
                let error_message = format!("Failed to get connection from pool: {}", e);
                return Err(CacheServiceError::ServerError(Some(error_message)));
            }
        };

        let mut cached_data: Vec<String> = Vec::new();

        if req.country.is_some() && req.state.is_some() {
            let cache_key = format!("weather:current:{}:{}:{}", req.country.clone().unwrap(),
                                    req.state.clone().unwrap(),
                                    req.location_name.clone().unwrap());
            let result: Option<String> = conn.get(&cache_key)
                .await
                .map_err(|e| {
                    CacheServiceError::RedisError(Some(e.code().unwrap_or("").to_string()), Some(e.to_string()))
                },
                )?;
            cached_data.push(result.unwrap());
        }
        else {
            let pattern =  format!(
            "weather:current:{}:{}:{}",
            req.country.clone().unwrap_or("*".to_string()),
            req.state.clone().unwrap_or("*".to_string()),
            req.location_name.clone().unwrap(),
        );
            let mut iter = conn.scan_match::<_, String>(pattern).await.map_err(|e| {
                CacheServiceError::RedisError(Some(e.code().unwrap_or("").to_string()), Some(e.to_string()))
            })?;

            let mut keys = Vec::new();
            while let Some(key) = iter.next_item().await {
                println!("key: {:?}", key);
                keys.push(key);
            }

            if keys.is_empty() {
                return Ok(Vec::new());
            }

            let mut conn = match redis_pool.get().await {
                Ok(c) => c,
                Err(e) => {
                    let error_message = format!("Failed to get connection from pool: {}", e);
                    return Err(CacheServiceError::ServerError(Some(error_message)));
                }
            };

            let values: Vec<Option<String>> = conn.mget(&keys).await
                .map_err(|e| {
                    CacheServiceError::RedisError(Some(e.code().unwrap_or("").to_string()), Some(e.to_string()))
                })?;
            cached_data = values.into_iter().flatten().collect();
        }

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

        let coord = Coord { x: data.lon, y: data.lat };
        let hash = encode(coord, 5).expect("Invalid coordinates"); //kao env
        let cache_key = format!("weather:current:{}", hash);
        let json = serde_json::to_string(&data.current_weather_data).unwrap();
        let result: RedisResult<()> = conn.set_ex(cache_key, json.clone(), 1800).await;

        match result {
            Ok(_) => {
                tracing::info!("Successfully stored current weather data by coordinates to redis store.");
            },
            Err(e) => {
                tracing::error!("Failed to store current weather data to redis store.");
                return Err(CacheServiceError::StoringCacheError(Some(e.to_string())));
            }
        };

        for location_name in &data.location_names {
            let cache_key = format!(
                "weather:current:{}:{}:{}",
                data.current_weather_data.location.country.clone().unwrap(),
                data.current_weather_data.location.state_region_province_or_entity.clone().unwrap_or("NoState".to_string()),
                location_name.clone(),

            );
            let result: RedisResult<()> = conn.set_ex(cache_key, json.clone(), 1800).await;
            match result {
                Ok(_) => {
                    tracing::info!(
                        "Successfully stored current weather data by location {} and country {} to redis store.",
                        location_name.clone(),
                        data.current_weather_data.location.country.clone().unwrap()
                    );
                },
                Err(e) => {
                    tracing::error!("Failed to store current weather data to redis store.");
                    return Err(CacheServiceError::StoringCacheError(Some(e.to_string())));
                }
            };

        }

        Ok(true)


    }
}

impl Default for CurrentWeatherRepository {
    fn default() -> Self {
        Self::new()
    }
}