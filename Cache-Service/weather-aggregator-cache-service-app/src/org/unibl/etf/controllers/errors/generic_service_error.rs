use std::fmt;
use actix_web::{error, HttpResponse};
use actix_web::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Serialize};
use crate::org::unibl::etf::model::errors::cache_service_error::CacheServiceError;
use crate::org::unibl::etf::util::serializers::format_milliseconds;

#[derive(Serialize, Debug, Clone)]
pub struct GenericServiceError {
    pub error: GenericServiceErrorDetails,
}

#[derive(Serialize, Debug, Clone)]
pub struct GenericServiceErrorDetails {
    pub code: CacheServiceError,
    pub code_numeric: u16,
    pub message: String,
    #[serde(serialize_with = "format_milliseconds")]
    pub timestamp: DateTime<Utc>,
}

impl GenericServiceErrorDetails {
    pub fn new_cache_error(e: CacheServiceError) -> Self {
        Self {
            code: e.clone(),
            code_numeric: e.as_numeric(),
            message: e.get_message(),
            timestamp: Utc::now(), //mozda lokal time!!!
        }
    }
}

impl fmt::Display for GenericServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error.message)
    }
}


impl error::ResponseError for GenericServiceError {
    fn status_code(&self) -> StatusCode {
        let status_code : StatusCode = match self.error.code {
            CacheServiceError::CacheMissError(_, _) => {
                StatusCode::NOT_FOUND
            },
            CacheServiceError::RequestValidationError(_) => {
                StatusCode::BAD_REQUEST
            },
            _ => StatusCode::INTERNAL_SERVER_ERROR
        };
        status_code
    }

    fn error_response(&self) -> HttpResponse {

        let mut sanitized_details = self.error.clone();

        // 2. Sanitize the clone
        sanitized_details.code = sanitized_details.code.get_sanitized_error();


        HttpResponse::build(self.status_code())
            .json(GenericServiceError {
                error: sanitized_details,
            })
    }
}


