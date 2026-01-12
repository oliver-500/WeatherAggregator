use serde::{Serialize};
use crate::org::unibl::etf::model::responses::current_weather_cache_response::CurrentWeatherCacheResponse;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CacheServiceError {
    ConnectionError(Option<String>),
    ResponseParsingError(Option<String>),
    ServerError(Option<String>),
    CacheMissError(Option<f64>, Option<f64>, Option<String>, Option<String>),
    RequestValidationError(Option<String>),
    RedisError(Option<String>, Option<String>),
    StoringCacheError(Option<String>),
    MultipleCacheResultsWithSameNameError(Vec<CurrentWeatherCacheResponse>)
}



impl CacheServiceError {

    pub fn get_sanitized_error(&self) -> CacheServiceError {
        match self {
            Self::CacheMissError(lat, lon, location_name, country) =>
                Self::CacheMissError(lat.clone(), lon.clone(), location_name.clone(), country.clone()),
            Self::RequestValidationError(err) => Self::RequestValidationError(err.clone()),
            Self::MultipleCacheResultsWithSameNameError(candidates) => Self::MultipleCacheResultsWithSameNameError(candidates.clone()),
            _ => Self::ServerError(None),
        }
    }

    pub fn get_sanitized_message(&self) -> String {
        match self {
            Self::CacheMissError(_, _, _, _) => self.get_message(),
            Self::RequestValidationError(s) => s.clone().unwrap_or(String::default()),
            _ => String::default(),
        }

    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConnectionError(_) => "CONNECTION_ERROR",
            Self::ResponseParsingError(_) => "RESPONSE_PARSING_ERROR",
            Self::ServerError(_) => "SERVER_ERROR",
            Self::CacheMissError(_, _, _, _) => "CACHE_MISS_ERROR",
            Self::RequestValidationError(_) => "REQUEST_VALIDATION_ERROR",
            Self::RedisError(_, _) => "REDIS_ERROR",
            Self::StoringCacheError(_) => "STORING_CACHE_ERROR",
            Self::MultipleCacheResultsWithSameNameError(_) => "MULTIPLE_CACHE_RESULTS_WITH_SAME_NAME_ERROR",
        }
    }


    pub fn get_message(&self) -> String {
        match self {
            CacheServiceError::ResponseParsingError(msg) => msg.clone().unwrap_or(String::default()),
            CacheServiceError::ConnectionError(msg) => msg.clone().unwrap_or(String::default()),
            CacheServiceError::ServerError(msg) => msg.clone().unwrap_or(String::default()),
            CacheServiceError::RequestValidationError(msg) => msg.clone().unwrap_or(String::default()),
            CacheServiceError::CacheMissError(lat, lon, location_name, country) => {
                if lat.is_some() && lon.is_some() {
                    format!("Current weather cache data for location with latitude {} and longitude {} not found.", lat.clone().unwrap(), lon.clone().unwrap())
                } else if country.is_some() && location_name.is_some() {
                    format!("Current weather cache data for location with name {} and country {} not found.", location_name.clone().unwrap(), country.clone().unwrap_or(String::default()))
                }
                else {
                    format!("Current weather cache data for location with name {} not found", country.clone().unwrap_or(String::default()))
                }

            },
            CacheServiceError::MultipleCacheResultsWithSameNameError(_results) => {
                format!("Multiple locations with the same name found")
            }
            CacheServiceError::RedisError(_, msg) => msg.clone().unwrap_or(String::default()),
            _ => { String::default() }
        }
    }

    pub fn as_numeric(&self) -> u16 {
        match self {
            CacheServiceError::CacheMissError(_, _, _, _) => 1000,
            CacheServiceError::RequestValidationError(_) => 1001,
            CacheServiceError::ServerError(_) => 1002,
            CacheServiceError::ConnectionError(_) => 1003,
            CacheServiceError::ResponseParsingError(_) => 1004,
            CacheServiceError::RedisError(_, _) => 1005,
            CacheServiceError::StoringCacheError(_) => 1007,
            CacheServiceError::MultipleCacheResultsWithSameNameError(_) => 1008,
        }
    }
}


