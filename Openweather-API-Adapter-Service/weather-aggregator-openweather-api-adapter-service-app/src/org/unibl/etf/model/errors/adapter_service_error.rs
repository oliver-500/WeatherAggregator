use serde::{Serialize};
use crate::org::unibl::etf::model::responses::geocoding_response::LocationCandidate;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdapterServiceError {
    ConnectionError,
    ExternalAPIResponseParsingError(Option<String>),
    ServerError(Option<String>),
    OpenWeatherAPIError(u16, Option<String>),
    LocationNotFoundError(Option<String>),
    ValidationError(Option<String>),
    InvalidProviderResponse(Option<String>),
    GeocodingError(u16, Option<String>),
    GeocodingResponseParsingError(Option<String>),
    AmbiguousLocationName(Vec<LocationCandidate>),
}

impl AdapterServiceError {
    pub fn get_message(&self) -> String {
        match self {
            AdapterServiceError::ExternalAPIResponseParsingError(msg) => msg.clone().unwrap_or(String::default()),
            AdapterServiceError::ConnectionError => "Failed to reach the provider.".to_string(),
            AdapterServiceError::ServerError(msg) => msg.clone().unwrap_or(String::default()),
            AdapterServiceError::OpenWeatherAPIError(status, message) => {
                if let Some(message) = message {
                    format!("Unexpected Geocoding API error with status {}: {}", status, message)
                } else {
                    String::default()
                }
            },
            AdapterServiceError::LocationNotFoundError(msg) => {
                format!("Location with name {} not found", msg.clone().unwrap_or(String::default()))
            }

            _ => { String::default() }
        }
    }

    pub fn as_numeric(&self) -> u16 {
        match self {
            AdapterServiceError::ConnectionError => 1000,
            AdapterServiceError::ExternalAPIResponseParsingError(_) => 1001,
            AdapterServiceError::ServerError(_) => 500,
            AdapterServiceError::OpenWeatherAPIError(error_code, _) => {
                match error_code {
                    400 => 1002,
                    401 => 1003,
                    403 => 1004,
                    404 => 1005,
                    429 => 1006,
                    _ => 1007,
                }
            },

            AdapterServiceError::LocationNotFoundError(_) => 1008,
            AdapterServiceError::ValidationError(_) => 1009,
            AdapterServiceError::InvalidProviderResponse(_) => 1010,
            AdapterServiceError::GeocodingError(_,_) => 1011,
            AdapterServiceError::GeocodingResponseParsingError(_) => 1012,
            AdapterServiceError::AmbiguousLocationName(_) => 1013,

        }
    }
}


