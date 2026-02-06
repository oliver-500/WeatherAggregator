use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserIdentityServiceError {
    RequestValidationError(Option<String>),
    ServerError(Option<String>),
    ConnectionError(Option<String>),
    ResponseParsingError(Option<String>),
    JwtCookieNotFoundError(Option<String>),
    TamperedJwtTokenError(Option<String>),
    DatabaseError(Option<String>),
    UserError(Option<String>),
    ApplicationCurrentlyUnavailable,
    Unauthorized(Option<String>),
    ExpiredRefreshTokenError(Option<String>),

}

impl UserIdentityServiceError {
    pub fn get_sanitized_error(&self) -> Self {
        match self {
            Self::RequestValidationError(s) => Self::RequestValidationError(s.clone()),
            Self::JwtCookieNotFoundError(s) => Self::JwtCookieNotFoundError(s.clone()),
            Self::TamperedJwtTokenError(s) => Self::TamperedJwtTokenError(s.clone()),
            Self::UserError(s) => Self::UserError(s.clone()),
            Self::ApplicationCurrentlyUnavailable => Self::ApplicationCurrentlyUnavailable,
            Self::Unauthorized(s) => Self::Unauthorized(s.clone()),
            Self::ExpiredRefreshTokenError(_) => Self::ExpiredRefreshTokenError(None),
            _ => Self::ServerError(None),
        }
    }
    pub fn get_sanitized_message(&self) -> String {
        match self {
            Self::RequestValidationError(s) => s.clone().unwrap_or(String::default()),
            Self::JwtCookieNotFoundError(s) => s.clone().unwrap_or(String::default()),
            Self::TamperedJwtTokenError(s) => s.clone().unwrap_or(String::default()),
            Self::UserError(s) => s.clone().unwrap_or(String::default()),
            Self::ApplicationCurrentlyUnavailable => "Application currently unavailable.".to_string(),
            Self::Unauthorized(s) => s.clone().unwrap_or(String::default()),
            Self::ExpiredRefreshTokenError(s) => s.clone().unwrap_or(String::default()),
            _ => String::default(),
        }

    }

    pub fn get_message(&self) -> String {
        match self {
            UserIdentityServiceError::RequestValidationError(msg) => {
                msg.clone().unwrap_or(String::from("RequestValidationError"))
            },
            UserIdentityServiceError::JwtCookieNotFoundError(msg) => {
                msg.clone().unwrap_or(String::from("JwtCookieNotFoundError"))
            },
            UserIdentityServiceError::TamperedJwtTokenError(msg) => {
                msg.clone().unwrap_or(String::from("TamperedJwtTokenError"))
            },
            UserIdentityServiceError::UserError(msg) => {
                msg.clone().unwrap_or(String::from("UserError"))
            },
            UserIdentityServiceError::ApplicationCurrentlyUnavailable => {
                String::from("Application currently unavailable.")
            },
            UserIdentityServiceError::Unauthorized(msg) => {
                msg.clone().unwrap_or(String::from("Unauthorized"))
            },
            UserIdentityServiceError::ExpiredRefreshTokenError(msg) => {
                msg.clone().unwrap_or(String::from("ExpiredRefreshTokenError"))
            }
            _ => { String::default() }
        }
    }

    pub fn as_numeric(&self) -> u16 {
        match self {
            UserIdentityServiceError::RequestValidationError(_) => 400,
            UserIdentityServiceError::JwtCookieNotFoundError(_) => 404,
            UserIdentityServiceError::TamperedJwtTokenError(_) => 400,
            UserIdentityServiceError::UserError(_) => 409,
            UserIdentityServiceError::ExpiredRefreshTokenError(_) => 403,
            UserIdentityServiceError::ServerError(_) => 500,

            UserIdentityServiceError::ConnectionError(_) => {
                500
            }
            UserIdentityServiceError::ResponseParsingError(_) => {
                500
            },
            UserIdentityServiceError::Unauthorized(_) => 401,
            _ => {
                500
            },
        }
    }
}
