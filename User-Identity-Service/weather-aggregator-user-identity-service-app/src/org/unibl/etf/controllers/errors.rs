use std::fmt;
use actix_web::{error, HttpResponse};
use actix_web::http::StatusCode;
use chrono::{DateTime, Utc};
use serde::{Serialize};
use crate::org::unibl::etf::controllers::auth_controller::build_logout_cookie;
use crate::org::unibl::etf::model::errors::user_identity_service_error::UserIdentityServiceError;

#[derive(Serialize, Debug, Clone)]
pub struct GenericServiceError {
    pub error: GenericServiceErrorDetails,
}

#[derive(Serialize, Debug, Clone)]
pub struct GenericServiceErrorDetails {
    pub code: UserIdentityServiceError,
    pub code_numeric: u16,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

impl GenericServiceErrorDetails {
    pub fn new_user_identity_service_error(e: UserIdentityServiceError) -> Self {
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
            UserIdentityServiceError::RequestValidationError(_) => {
                StatusCode::BAD_REQUEST
            },
            UserIdentityServiceError::UserError(_) => {
                StatusCode::CONFLICT
            },
            UserIdentityServiceError::DatabaseError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            },
            UserIdentityServiceError::TamperedJwtTokenError(_) => {
                StatusCode::UNAUTHORIZED
            },
            UserIdentityServiceError::Unauthorized(_) => {
                StatusCode::UNAUTHORIZED
            },
            UserIdentityServiceError::JwtCookieNotFoundError(_) => {
                StatusCode::BAD_REQUEST
            }
            _ => {
                StatusCode::INTERNAL_SERVER_ERROR
            },
        };
        status_code
    }

    fn error_response(&self) -> HttpResponse {
        let mut sanitized_details = self.error.clone();
        sanitized_details.message = sanitized_details.code.get_sanitized_message();

        // 2. Sanitize the clone
        sanitized_details.code = sanitized_details.code.get_sanitized_error();
        let mut response = HttpResponse::build(self.status_code());

        // Check for the specific error and add the cookie
        if let UserIdentityServiceError::TamperedJwtTokenError(_) = self.error.code {
            response.cookie(build_logout_cookie("access_token")).cookie(build_logout_cookie("refresh_token"));
            
        }

        response.json(GenericServiceError {
            error: sanitized_details,
        })
    }
}

