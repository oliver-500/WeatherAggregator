use actix_web::{HttpRequest, HttpResponse};
use actix_web_validator::Error;
use chrono::Utc;
use crate::org::unibl::etf::controllers::errors::{GenericServiceError, GenericServiceErrorDetails};
use crate::org::unibl::etf::model::errors::aggregator_error::AggregatorError;

pub fn handle_validation_error(err: Error, _req: &HttpRequest) -> actix_web::Error {

    let resp = GenericServiceError {
        error: GenericServiceErrorDetails {
            code: AggregatorError::ValidationError(err.to_string()),
            code_numeric: 400,
            message: "Validation of query parameters failed".to_string(),
            timestamp: Utc::now(),
        }
    };

    actix_web::error::InternalError::from_response(
        err,
        HttpResponse::BadRequest().json(resp),
    )
        .into()
}