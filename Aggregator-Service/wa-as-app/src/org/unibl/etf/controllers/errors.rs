use std::fmt;
use actix_web::{error, HttpResponse};
use actix_web::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Serialize};

use crate::org::unibl::etf::model::errors::aggregator_error::AggregatorError;
use crate::org::unibl::etf::model::errors::external_api_adapter_error_message::{AdapterError, ExternalAPIAdapterError};

#[derive(Serialize, Debug)]
pub struct GenericServiceError {
    pub error: GenericServiceErrorDetails,
}

#[derive(Serialize, Debug)]
pub struct GenericServiceErrorDetails {
    pub code: AggregatorError,
    pub code_numeric: u16,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

impl GenericServiceErrorDetails {
    pub fn new_aggregator_error(e: AggregatorError) -> Self {
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
            AggregatorError::ValidationError(_) => {
                StatusCode::BAD_REQUEST
            },
            _ => {
                StatusCode::INTERNAL_SERVER_ERROR
            },
        };
        status_code
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(&self)
    }
}

