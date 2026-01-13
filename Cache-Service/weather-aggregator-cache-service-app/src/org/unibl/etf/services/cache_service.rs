
use crate::org::unibl::etf::model::errors::cache_service_error::{CacheServiceError};
use crate::org::unibl::etf::model::requests::retrieve_current_weather_cache_request::RetrieveCurrentWeatherCacheRequest;
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
    pub async fn get_current_weather_cache_data_by_coordinates(
        &self,
        req: &RetrieveCurrentWeatherCacheRequest,
        redis_pool: &deadpool_redis::Pool,
    ) -> Result<CurrentWeatherCacheResponse, CacheServiceError> {
        let cached_data = match self.current_weather_repository
            .retrieve_current_weather_cache_result_by_coordinates(
                req,
                redis_pool
            )
            .await {
            Ok(cached_data) => {
                tracing::info!("Successfully retrieved current weather cache data {:?}", cached_data);
                cached_data
            },
            Err(e) => {
                tracing::info!("Was not able to get current weather cache data with error: {}", e.get_message());
                return Err(e);
            }
        };

        let cached_data = match cached_data {
            None => {
                return Err(CacheServiceError::CacheMissError(req.lat.clone(), req.lon.clone(), None, None));
            }
            Some(cached_data) => {
                cached_data
            }
        };

        let result: Result<CurrentWeatherCacheResponse, _> = serde_json::from_str(&cached_data);

        match result {
            Ok(data) => {
                Ok(data)
            }
            Err(e) => {
                Err(CacheServiceError::ResponseParsingError(Some(e.to_string())))
            }
        }
    }

    #[tracing::instrument(name = "Get Current Weather Cached Data Service", skip(redis_pool))]
    pub async fn get_current_weather_cache_data_by_location(
        &self,
        req: &RetrieveCurrentWeatherCacheRequest,
        redis_pool: &deadpool_redis::Pool,
    ) -> Result<CurrentWeatherCacheResponse, CacheServiceError> {
        match self.current_weather_repository
            .retrieve_current_weather_cache_result_by_location(
                req,
                redis_pool
            )
            .await {
            Ok(cached_data) => {
                tracing::info!("Successfully retrieved current weather cache data {:?}", cached_data);
                let cached_data = match cached_data {
                    None => {
                        return Err(CacheServiceError::ResponseParsingError(Some("Error while parsing repository response".to_string())));
                    }
                    Some(data) => {
                        data
                    }
                };

                let result: Result<CurrentWeatherCacheResponse, _> = serde_json::from_str(&cached_data);
                match result {
                    Ok(data) => {
                        return Ok(data);
                    }
                    Err(e) => {
                        tracing::info!("Error while parsing one of results. Item: {}, Error: {}", cached_data, e.to_string());
                        return Err(CacheServiceError::ResponseParsingError(Some("Failed to parse response".to_string())));
                    }
                };
            },
            Err(e) => {
                match e {
                    CacheServiceError::MultipleCachedResultsError(cached_data) => {
                        let mut candidates: Vec<CurrentWeatherCacheResponse> = Vec::new();

                        for item in cached_data {
                            let result: Result<CurrentWeatherCacheResponse, _> = serde_json::from_str(&item);
                            match result {
                                Ok(data) => {
                                    candidates.push(data);
                                }
                                Err(e) => {
                                    tracing::info!("Error while parsing one of results. Item: {}, Error: {}", item, e.to_string())
                                }
                            }
                        }
                        return Err(CacheServiceError::OnlyPotentialMatchesFoundError(candidates));
                    },
                    _ => {
                        tracing::info!("Was not able to get current weather cache data with error: {}", e.get_message());
                        return Err(e);
                    }
                }
            }
        };
    }

    #[tracing::instrument(name = "Store Current Weather Data Cache Service",
        skip(redis_pool))]
    pub async fn store_current_weather_result_as_cache (
        &self,
        req: &StoreCurrentWeatherDataRequest,
        redis_pool: &deadpool_redis::Pool,
    ) -> Result<bool, CacheServiceError> {
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