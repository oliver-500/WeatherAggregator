use serde::{Serialize};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdapterServiceError {
    ConnectionError(Option<String>),
    ExternalAPIResponseParsingError(Option<String>),
    ServerError(Option<String>),
    WeatherAPIError(u16, Option<String>),
    LocationNotFoundError(String),
    RequestParametersValidationError(Option<String>),
    InvalidProviderResponseError(Option<String>),
}

impl AdapterServiceError {


    pub fn get_sanitized_error(&self) -> Self {

        match self {
            Self::LocationNotFoundError(s) => Self::LocationNotFoundError(s.clone()),
            Self::RequestParametersValidationError(s) => Self::RequestParametersValidationError(s.clone()),
            _ => Self::ServerError(None),
        }

    }


    pub fn get_message(&self) -> String {
        match self {
            AdapterServiceError::ExternalAPIResponseParsingError(msg) => msg.clone().unwrap_or(String::default()),
            AdapterServiceError::ConnectionError(msg) => msg.clone().unwrap_or(String::default()),
            AdapterServiceError::ServerError(msg) => msg.clone().unwrap_or(String::default()),
            AdapterServiceError::WeatherAPIError(status, message) => {
                if let Some(message) = message {
                    format!("Unexpected Geocoding API error with status {}: {}", status, message)
                } else {
                    String::default()
                }
            },
            AdapterServiceError::LocationNotFoundError(location) => {
                format!("Location with name {} not found", location)
            }

            _ => { String::default() }
        }
    }

    pub fn as_numeric(&self) -> u16 {
        match self {
            AdapterServiceError::ConnectionError(_) => 1000,
            AdapterServiceError::ExternalAPIResponseParsingError(_) => 1001,
            AdapterServiceError::ServerError(_) => 500,
            AdapterServiceError::WeatherAPIError(weather_api_error_code, _) => {
                match weather_api_error_code {
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
            AdapterServiceError::LocationNotFoundError(_) => 1016,
            AdapterServiceError::RequestParametersValidationError(_) => 1017,
            AdapterServiceError::InvalidProviderResponseError(_) => 1018,
        }
    }
}


