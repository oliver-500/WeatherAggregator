use actix_web::{HttpRequest, HttpResponse};
use actix_web_validator::Error;
use chrono::Utc;
use crate::org::unibl::etf::controllers::errors::{GenericServiceError, GenericServiceErrorDetails};
use crate::org::unibl::etf::model::errors::user_identity_service_error::UserIdentityServiceError;

pub fn handle_validation_error(err: Error, _req: &HttpRequest) -> actix_web::Error {

    let aggregator_error = UserIdentityServiceError::RequestValidationError(Some(err.to_string()));
    let resp = GenericServiceError {
        error: GenericServiceErrorDetails {
            code: aggregator_error.clone(),
            code_numeric: aggregator_error.as_numeric(),
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