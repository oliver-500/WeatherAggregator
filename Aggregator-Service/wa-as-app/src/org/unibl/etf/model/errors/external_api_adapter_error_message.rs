use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ExternalAPIAdapterError {
    pub error: ExternalAPIAdapterErrorDetails,
}

#[derive(Deserialize, Debug)]
pub struct ExternalAPIAdapterErrorDetails {
    pub code: AdapterError,
    pub code_numeric: u16,
    pub message: String,
    pub provider: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdapterError {
    // This matches "AMBIGUOUS_LOCATION": [...]
    AmbiguousLocation(Vec<LocationCandidate>),

    // You can add other error types here later
    InvalidProviderResponse(String),
    ApiResponseParsingError(String),
    ConnectionError(String),
    ExternalApiError(u16, Option<String>),

    ServerError(Option<String>),
    ValidationError,
    LocationNotFound(String),
    UnknownProvider,
    EndpointNotFound,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LocationCandidate {
    pub location: String,
    pub state: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
}