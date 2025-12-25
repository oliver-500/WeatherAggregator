use serde::Serialize;
use crate::org::unibl::etf::model::errors::ambiguous_location_error::Candidate;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdapterError {
    InvalidProviderResponse(String),
    ApiResponseParsingError(String),
    ConnectionError(String),
    ExternalApiError(u16, Option<String>),
    AmbiguousLocation(Vec<Candidate>),
    ServerError(Option<String>),
    ValidationError,
    LocationNotFound(String),
    UnknownProvider,
    EndpointNotFound,
}

impl AdapterError {
    pub fn get_message(&self) -> String {
        match self {
            AdapterError::InvalidProviderResponse(msg) => msg.clone(),
            AdapterError::ConnectionError(msg) => msg.clone(),
            AdapterError::ExternalApiError(_, msg) => msg.clone().unwrap_or(String::default()),
            AdapterError::ApiResponseParsingError(msg) => msg.clone(),
            AdapterError::ServerError(msg) => msg.clone().unwrap_or(String::default()),
            AdapterError::ValidationError => "Validation error".to_string(),
            AdapterError::LocationNotFound(_) => "Location not found".to_string(),
            AdapterError::AmbiguousLocation(_) => "There are multiple locations with the provided name. Choose one of them.".to_string(),
            _ => { "".to_string() }
        }
    }

    pub fn as_numeric(&self) -> u16 {
        match self {
            AdapterError::InvalidProviderResponse(_) => 1000,
            AdapterError::ConnectionError(_) => 1001,
            AdapterError::ApiResponseParsingError(_) => 1002,
            AdapterError::ExternalApiError(external_api_error_code, _) => {
                match external_api_error_code {
                    1002 => 1003,
                    1003 => 1004,
                    1005 => 1005,
                    1006 => 1006,
                    2006 => 1007,
                    2007 => 1008,
                    2008 => 1009,
                    2009 => 1011,
                    9000 => 1012,
                    9001 => 1013,
                    9999 => 1014,
                    _ => 1015,


                }

            },
            AdapterError::ServerError(_) => 500,
            AdapterError::ValidationError => 400,
            AdapterError::AmbiguousLocation(_) => 400,
            AdapterError::LocationNotFound(_) => 401,
            AdapterError::UnknownProvider => 400,
            AdapterError::EndpointNotFound => 404,
        }
    }
}