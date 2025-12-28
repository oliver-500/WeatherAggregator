use serde::{Serialize};


#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GeocodingServiceError {
    ConnectionError(Option<String>),
    ResponseParsingError(Option<String>),
    ServerError(Option<String>),
    ExternalGeocodingApiError(u16, Option<String>),
    LocationNotFoundError(Option<String>),
    ValidationError(Option<String>),
}

impl GeocodingServiceError {
    pub fn get_message(&self) -> String {
        match self {
            GeocodingServiceError::ResponseParsingError(msg) => msg.clone().unwrap_or(String::default()),
            GeocodingServiceError::ConnectionError(msg) => msg.clone().unwrap_or(String::default()),
            GeocodingServiceError::ServerError(msg) => msg.clone().unwrap_or(String::default()),
            GeocodingServiceError::ExternalGeocodingApiError(status, message) => {
                if let Some(message) = message {
                    format!("Unexpected Geocoding API error with status {}: {}", status, message)
                } else {
                    String::default()
                }
            },
            GeocodingServiceError::LocationNotFoundError(msg) => {
                format!("Location with name {} not found", msg.clone().unwrap_or(String::default()))
            }

            _ => { String::default() }
        }
    }

    pub fn as_numeric(&self) -> u16 {
        match self {
            GeocodingServiceError::ConnectionError(_) => 1000,
            GeocodingServiceError::ResponseParsingError(_) => 1001,
            GeocodingServiceError::ServerError(_) => 500,
            GeocodingServiceError::ExternalGeocodingApiError(error_code, _) => {
                match error_code {
                    400 => 1002,
                    401 => 1003,
                    403 => 1004,
                    404 => 1005,
                    429 => 1006,
                    _ => 1007,
                }
            },

            GeocodingServiceError::LocationNotFoundError(_) => 1008,
            GeocodingServiceError::ValidationError(_) => 1009,
        }
    }
}


