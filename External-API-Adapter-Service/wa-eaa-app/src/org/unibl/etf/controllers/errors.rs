
use actix_web::{HttpResponse};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Serialize};
use crate::org::unibl::etf::model::errors::adapter_error::AdapterError;

pub trait HttpMappedError {
    fn to_response(&self) -> HttpResponse;
}


#[derive(Serialize)]
pub struct GenericServiceError {
    pub error: GenericServiceErrorDetails,
}

#[derive(Serialize)]
pub struct GenericServiceErrorDetails {
    pub code: AdapterError,
    pub code_numeric: u16,
    pub message: String,
    pub provider: String,
    pub timestamp: DateTime<Utc>,
}
impl HttpMappedError for GenericServiceError {
    fn to_response(&self) -> HttpResponse {
        let _headers: Vec<(HeaderName, HeaderValue)> = Vec::new();

        let status_code : StatusCode = match self.error.code {
            AdapterError::InvalidProviderResponse(_) => {
                StatusCode::INTERNAL_SERVER_ERROR

            }
            AdapterError::ApiResponseParsingError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            AdapterError::ConnectionError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            AdapterError::ExternalApiError(_, _) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            _ => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        HttpResponse::build(status_code).json(self)

    }
}