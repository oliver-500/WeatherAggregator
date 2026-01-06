use serde::{Serialize};
use crate::org::unibl::etf::model::errors::geocoding_error::{GeocodingServiceError};
use crate::org::unibl::etf::model::responses::geocoding_response::LocationCandidate;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdapterServiceError {
    ConnectionError(Option<String>),
    ExternalAPIResponseParsingError(Option<String>),
    ServerError(Option<String>),
    OpenWeatherAPIError(u16, Option<String>),
    LocationNotFoundError(Option<String>),
    RequestParametersValidationError(Option<String>),
    InvalidProviderResponseError(Option<String>),
    GeocodingServiceError(u16, Option<String>),
    GeocodingResponseParsingError(Option<String>),
    AmbiguousLocationNameError(Vec<LocationCandidate>),
    RateLimitExceeded,
    RedisError(Option<String>, Option<String>),
}

impl AdapterServiceError {

    pub fn get_sanitized_error(&self) -> Self {

        match self {
            Self::LocationNotFoundError(s) => Self::LocationNotFoundError(s.clone()),
            Self::AmbiguousLocationNameError(candidates) => Self::AmbiguousLocationNameError(candidates.clone()),
            Self::RequestParametersValidationError(s) => Self::RequestParametersValidationError(s.clone()),
            Self::RateLimitExceeded => Self::RateLimitExceeded,
            _ => Self::ServerError(None),
        }

    }

    pub fn get_sanitized_message(&self) -> String {
        match self {
            Self::LocationNotFoundError(s) => s.clone().unwrap_or(String::default()),
            Self::AmbiguousLocationNameError(_s) => self.get_message(),
            Self::RequestParametersValidationError(s) => s.clone().unwrap_or(String::default()),
            Self::RateLimitExceeded => self.get_message(),
            _ => String::default(),
        }

    }

    pub fn get_message(&self) -> String {
        match self {
            AdapterServiceError::LocationNotFoundError(msg) => {
                format!("Location with name {} not found", msg.clone().unwrap_or(String::default()))
            },
            AdapterServiceError::AmbiguousLocationNameError(_) => {
                String::from("There are multiple locations with the provided name. Choose one of them.")
            },
            AdapterServiceError::RequestParametersValidationError(_) => {
                String::from("Request parameters are invalid.")
            },
            AdapterServiceError::RateLimitExceeded => {
                String::from("API Rate Limit exceeded for the provider.")
            }
            _ => { String::from("Unexpected server error.") }
        }
    }

    pub fn as_numeric(&self) -> u16 {
        match self {
            AdapterServiceError::ServerError(_) => 1000,
            AdapterServiceError::OpenWeatherAPIError(error_code, _) => {
                match error_code {
                    400 => 1001,
                    401 => 1002,
                    403 => 1003,
                    404 => 1004,
                    429 => 1005,
                    _ => 1006,
                }
            },
            AdapterServiceError::LocationNotFoundError(_) => 1008,
            AdapterServiceError::AmbiguousLocationNameError(_) => 1009,
            AdapterServiceError::RequestParametersValidationError(_) => 1010,
            AdapterServiceError::RateLimitExceeded => 1011,
            AdapterServiceError::InvalidProviderResponseError(_) => 1012,
            AdapterServiceError::GeocodingServiceError(_, _) => 1013,
            AdapterServiceError::GeocodingResponseParsingError(_) => 1014,
            AdapterServiceError::RedisError(_, _) => 1015,
            _ => 1016,

        }
    }
}

impl From<GeocodingServiceError> for AdapterServiceError {
    fn from(code: GeocodingServiceError) -> Self {
        match code {
            GeocodingServiceError::LocationNotFoundError(s) => {
                AdapterServiceError::LocationNotFoundError(s)
            }
            GeocodingServiceError::ServerError => {
                AdapterServiceError::ServerError(None)
            },
            GeocodingServiceError::RateLimitExceeded => {
                AdapterServiceError::RateLimitExceeded
            }
        }
    }
}

