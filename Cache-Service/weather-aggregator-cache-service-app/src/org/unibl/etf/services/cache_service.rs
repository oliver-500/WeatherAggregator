
use crate::org::unibl::etf::model::errors::cache_service_error::{CacheServiceError};
use crate::org::unibl::etf::model::requests::current_weather_cache_request::CurrentWeatherCacheRequest;
use crate::org::unibl::etf::model::requests::store_current_weather_data_request::StoreCurrentWeatherDataRequest;
use crate::org::unibl::etf::model::responses::current_weather_cache_response::{CurrentWeatherCacheResponse};
use crate::org::unibl::etf::repositories::current_weather_cache_repository::CurrentWeatherRepository;

#[derive(Debug)]
pub struct CacheService {
    current_weather_repository: CurrentWeatherRepository
}

impl CacheService {
    fn new() -> Self {
        Self {
            current_weather_repository: CurrentWeatherRepository::default()
        }
    }

    #[tracing::instrument(name = "Get Current Weather Cached Data Service", skip(redis_pool))]
    pub async fn get_current_weather_cached_result(
        &self,
        req: &CurrentWeatherCacheRequest,
        redis_pool: &deadpool_redis::Pool,
    ) -> Result<CurrentWeatherCacheResponse, CacheServiceError> {

        let cached_data = match self.current_weather_repository
            .get_current_weather_cache_result(
                req.lat,
                req.lon,
                redis_pool
            )
            .await {
            Ok(cached_data) => {
                tracing::info!("Succesfully retrieved current weather cache data {:?}", cached_data);
                cached_data
            },
            Err(e) => {
                tracing::info!("Was not able to get current weather cache data with error: {}", e.get_message());
                return Err(e);
            }
        };

        let cached_data_json = match cached_data {
            None => {
                tracing::info!("Cache miss. No cached weather data for such request");
                return Err(CacheServiceError::CacheMissError(req.lat, req.lon));
            }
            Some(json) => {
                let result: Result<CurrentWeatherCacheResponse, _> = serde_json::from_str(&json);
                result
            }
        };

        match cached_data_json {
            Ok(data) => {
                Ok(data)
            }
            Err(e) => {
                Err(CacheServiceError::ResponseParsingError(Some(e.to_string())))
            }
        }
    }


    #[tracing::instrument(name = "Store Current Weather Data Cache Service",
        skip(redis_pool))]
    pub async fn store_current_weather_result_as_cache (
        &self,
        req: &StoreCurrentWeatherDataRequest,
        redis_pool: &deadpool_redis::Pool,
    ) -> Result<bool, CacheServiceError>
    {
        self.current_weather_repository
            .store_current_weather_result_as_cache(
                req,
                redis_pool
            )
            .await
    }
}

impl Default for CacheService {
    fn default() -> Self {
        Self::new()
    }
}