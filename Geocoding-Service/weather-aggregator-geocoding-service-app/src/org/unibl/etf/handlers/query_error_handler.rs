use actix_web::{HttpRequest, HttpResponse};
use actix_web_validator::Error;
use chrono::Utc;
use crate::org::unibl::etf::controllers::errors::generic_service_error::{GenericServiceError, GenericServiceErrorDetails};
use crate::org::unibl::etf::model::errors::geocoding_service_error::GeocodingServiceError;

pub fn handle_validation_error(err: Error, _req: &HttpRequest) -> actix_web::Error {
    let resp = GenericServiceError {
        error: GenericServiceErrorDetails {
            code: GeocodingServiceError::ValidationError(Some(err.to_string())),
            code_numeric: 400,
            message: "Validation of provided query parameters failed.".to_string(),
            timestamp: Utc::now(),
        }
    };

    actix_web::error::InternalError::from_response(
        err,
        HttpResponse::BadRequest().json(resp),
    )
        .into()
}