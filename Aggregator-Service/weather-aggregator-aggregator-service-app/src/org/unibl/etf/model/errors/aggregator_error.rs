use serde::{Deserialize, Serialize};
use crate::org::unibl::etf::model::errors::cache_service_error::CacheError;
use crate::org::unibl::etf::model::errors::external_api_adapter_error_message::{AdapterError, LocationCandidate};

#[derive(Serialize, Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AggregatorError {
    AmbiguousLocationNameError(Vec<LocationCandidate>), // Keep this specific for the UI
    RequestParametersValidationError(Option<String>),
    ServerError(Option<String>),
    LocationNotFoundError(Option<String>),
    ConnectionError(Option<String>),
    ResponseParsingError(Option<String>),
    WeatherDataUnavailableError,
    CacheMissError(f64, f64),
    StoringCacheError(Option<String>),

}

impl AggregatorError {

    pub fn get_sanitized_error(&self) -> Self {
        match self {
            Self::LocationNotFoundError(s) => Self::LocationNotFoundError(s.clone()),
            Self::RequestParametersValidationError(s) => Self::RequestParametersValidationError(s.clone()),
            Self::AmbiguousLocationNameError(s) => Self::AmbiguousLocationNameError(s.clone()),
            _ => Self::ServerError(None),
        }
    }
    pub fn get_sanitized_message(&self) -> String {
        match self {
            Self::LocationNotFoundError(s) => s.clone().unwrap_or(String::default()),
            Self::RequestParametersValidationError(s) => s.clone().unwrap_or(String::default()),
            Self::AmbiguousLocationNameError(_s) => self.get_message(),
            _ => String::default(),
        }

    }

    pub fn get_message(&self) -> String {
        match self {
            AggregatorError::RequestParametersValidationError(msg) => {
                msg.clone().unwrap_or(String::from("RequestParametersValidationError"))
            },
            AggregatorError::AmbiguousLocationNameError(_) => {
                String::from("AmbiguousLocationNameError")
            },
            AggregatorError::LocationNotFoundError(s) => format!("LocationNotFoundError: {}", s.clone().unwrap_or(String::from(""))),
            AggregatorError::WeatherDataUnavailableError => {
                String::from("No provider could return weather data")
            },
            //AggregatorError::ServerError(s) => format!("ServerError: {}", s.clone().unwrap_or(String::from(""))),
            AggregatorError::ResponseParsingError(s) => format!("ResponseParsingError: {}", s.clone().unwrap_or(String::from(""))),
            _ => { String::default() }
        }
    }

    pub fn as_numeric(&self) -> u16 {
        match self {
            AggregatorError::RequestParametersValidationError(_) => 400,
            AggregatorError::ServerError(_) => 500,
            AggregatorError::AmbiguousLocationNameError(_) => 409,
            AggregatorError::LocationNotFoundError(_) => 404,
            AggregatorError::ConnectionError(_) => {
                500
            }
            AggregatorError::ResponseParsingError(_) => {
                500
            },
            AggregatorError::WeatherDataUnavailableError => {
                404
            }
            _ => {
                500
            }
        }
    }
}

impl From<AdapterError> for AggregatorError {
    fn from(code: AdapterError) -> Self {
        match code {

            AdapterError::AmbiguousLocationNameError(candidates) => {
                AggregatorError::AmbiguousLocationNameError(candidates)
            }
            AdapterError::LocationNotFoundError(s) => {
                AggregatorError::LocationNotFoundError(s)
            }
            AdapterError::ServerError => {
                AggregatorError::ServerError(None)
            }

        }
    }
}

impl From<CacheError> for AggregatorError {
    fn from(code: CacheError) -> Self {
        match code {
            CacheError::CacheMissError(lat, lon) => {
                AggregatorError::CacheMissError(lat, lon)
            }
            CacheError::RequestValidationError(s) => {
                AggregatorError::RequestParametersValidationError(s)
            }
            CacheError::ServerError => {
                AggregatorError::ServerError(None)
            },
            CacheError::StoringCacheError(s) => {
                AggregatorError::StoringCacheError(s)
            }
        }
    }
}