
use crate::org::unibl::etf::model::errors::cache_service_error::{CacheServiceError};
use crate::org::unibl::etf::model::requests::current_weather_cache_request::CurrentWeatherCacheRequest;
use crate::org::unibl::etf::model::responses::current_weather_cache_response::{CurrentWeatherCacheResponse};
use crate::org::unibl::etf::repositories::current_weather_cache_repository::CurrentWeatherRepository;

pub struct CacheService {
    current_weather_repository: CurrentWeatherRepository
}

impl CacheService {
    fn new() -> Self {
        Self {
            current_weather_repository: CurrentWeatherRepository::default()
        }
    }
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
                cached_data
            },
            Err(e) => {
                return Err(e);
            }
        };

        let cached_data_json = match cached_data {
            None => {
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


    pub async fn store_current_weather_result_as_cache (
        &self,
        req: &CurrentWeatherCacheResponse,
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