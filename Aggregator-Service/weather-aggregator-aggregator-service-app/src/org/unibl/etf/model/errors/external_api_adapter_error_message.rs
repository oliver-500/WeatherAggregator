use crate::org::unibl::etf::util::deserializers::deserialize_error_code;
use crate::org::unibl::etf::util::deserializers::deserialize_timestamp;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
pub enum RawAdapterError {
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


