use serde::{Serialize};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CacheServiceError {
    ConnectionError(Option<String>),
    ResponseParsingError(Option<String>),
    ServerError(Option<String>),
    CacheMissError(f64, f64),
    RequestValidationError(Option<String>),
    RedisError(Option<String>, Option<String>),
}



impl CacheServiceError {

    pub fn get_sanitized_error(&self) -> CacheServiceError {
        match self {
            Self::CacheMissError(lat, lon) => Self::CacheMissError(lat.clone(), lon.clone()),
            Self::RequestValidationError(err) => Self::RequestValidationError(err.clone()),
            _ => Self::ServerError(None),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ConnectionError(_) => "CONNECTION_ERROR",
            Self::ResponseParsingError(_) => "RESPONSE_PARSING_ERROR",
            Self::ServerError(_) => "SERVER_ERROR",
            Self::CacheMissError(_, _) => "CACHE_MISS_ERROR",
            Self::RequestValidationError(_) => "REQUEST_VALIDATION_ERROR",
            Self::RedisError(_, _) => "REDIS_ERROR",
        }
    }


    pub fn get_message(&self) -> String {
        match self {
            CacheServiceError::ResponseParsingError(msg) => msg.clone().unwrap_or(String::default()),
            CacheServiceError::ConnectionError(msg) => msg.clone().unwrap_or(String::default()),
            CacheServiceError::ServerError(msg) => msg.clone().unwrap_or(String::default()),
            CacheServiceError::RequestValidationError(msg) => msg.clone().unwrap_or(String::default()),
            CacheServiceError::CacheMissError(lat, lon) => {
                format!("Current weather cache data for location with latitude {} and longitude {} not found.", lat, lon)
            },
            CacheServiceError::RedisError(_, msg) => msg.clone().unwrap_or(String::default()),
            _ => { String::default() }
        }
    }

    pub fn as_numeric(&self) -> u16 {
        match self {
            CacheServiceError::CacheMissError(_, _) => 1000,
            CacheServiceError::RequestValidationError(_) => 1001,
            CacheServiceError::ServerError(_) => 1002,
            CacheServiceError::ConnectionError(_) => 1003,
            CacheServiceError::ResponseParsingError(_) => 1004,
            CacheServiceError::RedisError(_, _) => 1005,
            _ => 1006,
        }
    }
}


