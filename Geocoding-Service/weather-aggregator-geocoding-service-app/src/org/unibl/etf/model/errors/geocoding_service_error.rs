use serde::{Serialize};


#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GeocodingServiceError {
    ConnectionError(Option<String>),
    ResponseParsingError(Option<String>),
    ServerError(Option<String>),
    ExternalGeocodingApiError(u16, Option<String>),
    LocationNotFoundError(Option<String>),
    RequestValidationError(Option<String>),
    RateLimitExceeded,
    RedisError(Option<String>, Option<String>),
}



impl GeocodingServiceError {

    pub fn get_sanitized_error(&self) -> GeocodingServiceError {
        match self {
            Self::LocationNotFoundError(s) => Self::LocationNotFoundError(s.clone()),
            Self::RequestValidationError(s) => Self::RequestValidationError(s.clone()),
            Self::RateLimitExceeded => Self::RateLimitExceeded,
            _ => Self::ServerError(None),
        }
    }
    pub fn get_sanitized_message(&self) -> String {
        match self {
            Self::LocationNotFoundError(s) => s.clone().unwrap_or(String::default()),
            Self::RequestValidationError(s) => s.clone().unwrap_or(String::default()),
            Self::RateLimitExceeded => self.get_message(),
            _ => String::default(),
        }

    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConnectionError(_) => "CONNECTION_ERROR",
            Self::ResponseParsingError(_) => "RESPONSE_PARSING_ERROR",
            Self::ServerError(_) => "SERVER_ERROR",
            Self::ExternalGeocodingApiError(_, _) => "EXTERNAL_GEOCODING_API_ERROR",
            Self::LocationNotFoundError(_) => "LOCATION_NOT_FOUND_ERROR",
            Self::RequestValidationError(_) => "REQUEST_VALIDATION_ERROR",
            Self::RateLimitExceeded => "REQUEST_RATE_LIMIT EXCEEDED",
            Self::RedisError(_, _) => "REDIS_ERROR",
        }
    }


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
            },
            GeocodingServiceError::RequestValidationError(msg) => {
                msg.clone().unwrap_or(String::default())
            },
            GeocodingServiceError::RateLimitExceeded => {
                format!("Rate limit exceeded for the geocoding provider.")
            },
            GeocodingServiceError::RedisError(_, msg) => msg.clone().unwrap_or(String::default()),

            // _ => { String::default() }
        }
    }

    pub fn as_numeric(&self) -> u16 {
        match self {
            GeocodingServiceError::LocationNotFoundError(_) => 1000,
            GeocodingServiceError::RequestValidationError(_) => 1001,
            GeocodingServiceError::RateLimitExceeded => 1002,
            GeocodingServiceError::RedisError(_, _) => 1003,
            _ => 1004,
        }
    }
}


