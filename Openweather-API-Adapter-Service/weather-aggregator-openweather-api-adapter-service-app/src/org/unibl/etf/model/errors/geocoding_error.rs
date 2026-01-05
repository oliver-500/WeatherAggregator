use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct GeocodingGenericError {
    pub error: GeocodingGenericErrorDetails,
}

#[derive(Deserialize, Debug)]
pub struct GeocodingGenericErrorDetails {
    #[serde(deserialize_with = "deserialize_error_code")]
    pub code: GeocodingServiceError,
    pub code_numeric: u16,
    pub message: String,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum GeocodingServiceError {
    LocationNotFoundError(Option<String>),
    RateLimitExceeded,
    ServerError,
}

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum RawRemoteError {
    LocationNotFoundError(Option<String>),
    RateLimitExceeded,
    ServerError,
    #[serde(other)]
    Unknown,
}

fn deserialize_error_code<'de, D>(deserializer: D) -> Result<GeocodingServiceError, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = RawRemoteError::deserialize(deserializer)?;
    match raw {
        RawRemoteError::LocationNotFoundError(msg) => {
            Ok(GeocodingServiceError::LocationNotFoundError(msg))
        }
        RawRemoteError::Unknown => {
            Ok(GeocodingServiceError::ServerError)
        },
        RawRemoteError::ServerError => {
            Ok(GeocodingServiceError::ServerError)
        },
        RawRemoteError::RateLimitExceeded => {
            Ok(GeocodingServiceError::RateLimitExceeded)
        }
    }
}

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    // Parses ISO 8601 strings like "2025-12-27T08:54:42.367Z"
    s.parse::<DateTime<Utc>>().map_err(serde::de::Error::custom)
}