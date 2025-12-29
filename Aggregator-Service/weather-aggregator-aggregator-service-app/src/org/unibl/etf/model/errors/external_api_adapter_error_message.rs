use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize)]
pub struct AdapterServiceError {
    pub error: AdapterServiceErrorDetails,
}

#[derive(Deserialize, Debug)]
pub struct AdapterServiceErrorDetails {
    #[serde(deserialize_with = "deserialize_error_code")]
    pub code: AdapterError,
    pub code_numeric: u16,
    pub message: String,
    pub provider: String,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum AdapterError {
    // This matches "AMBIGUOUS_LOCATION": [...]
    AmbiguousLocationNameError(Vec<LocationCandidate>),
    LocationNotFoundError(Option<String>),
    ServerError,
}

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum RawAdapterError {
    LocationNotFoundError(Option<String>),
    AmbiguousLocationNameError(Vec<LocationCandidate>),
    ServerError,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LocationCandidate {
    pub location_name: String,
    pub state: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
}

fn deserialize_error_code<'de, D>(deserializer: D) -> Result<AdapterError, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = RawAdapterError::deserialize(deserializer)?;
    match raw {
        RawAdapterError::LocationNotFoundError(msg) => {
            Ok(AdapterError::LocationNotFoundError(msg))
        },
        RawAdapterError::AmbiguousLocationNameError(candidates) => {
            Ok(AdapterError::AmbiguousLocationNameError(candidates))
        },
        RawAdapterError::Unknown => {
            Ok(AdapterError::ServerError)
        },
        RawAdapterError::ServerError => {
            Ok(AdapterError::ServerError)
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