use crate::org::unibl::etf::util::deserializers::deserialize_timestamp;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};
use crate::org::unibl::etf::model::responses::current_weather_response::CurrentWeatherResponse;

#[derive(Debug, Deserialize)]
pub struct CacheServiceError {
    pub error: CacheServiceErrorDetails,
}

#[derive(Deserialize, Debug)]
pub struct CacheServiceErrorDetails {
    #[serde(deserialize_with = "deserialize_error_code")]
    pub code: CacheError,
    pub code_numeric: u16,
    pub message: String,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum CacheError {
    CacheMissError(f64, f64),
    RequestValidationError(Option<String>),
    StoringCacheError(Option<String>),
    MultipleCacheResultsWithSameNameError(Vec<CurrentWeatherResponse>),
    ServerError,
}

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum RawCacheError {
    CacheMissError(f64, f64),
    RequestValidationError(Option<String>),
    StoringCacheError(Option<String>),
    ServerError,
    MultipleCacheResultsWithSameNameError(Vec<CurrentWeatherResponse>),
    #[serde(other)]
    Unknown,
}


fn deserialize_error_code<'de, D>(deserializer: D) -> Result<CacheError, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = RawCacheError::deserialize(deserializer)?;
    match raw {
        RawCacheError::CacheMissError(lat, lon) => {
            Ok(CacheError::CacheMissError(lat, lon))
        },
        RawCacheError::RequestValidationError(msg) => {
            Ok(CacheError::RequestValidationError(msg))
        },
        RawCacheError::Unknown => {
            Ok(CacheError::ServerError)
        },
        RawCacheError::ServerError => {
            Ok(CacheError::ServerError)
        },
        RawCacheError::StoringCacheError(msg) => {
            Ok(CacheError::StoringCacheError(msg))
        },
        RawCacheError::MultipleCacheResultsWithSameNameError(results) => {
            Ok(CacheError::MultipleCacheResultsWithSameNameError(results))
        }
    }
}

