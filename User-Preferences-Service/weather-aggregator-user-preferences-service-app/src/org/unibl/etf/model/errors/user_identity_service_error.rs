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

}

impl UserIdentityServiceError {
    pub fn get_sanitized_error(&self) -> Self {
        match self {
            Self::RequestValidationError(s) => Self::RequestValidationError(s.clone()),
            Self::JwtCookieNotFoundError(s) => Self::JwtCookieNotFoundError(s.clone()),
            Self::TamperedJwtTokenError(s) => Self::TamperedJwtTokenError(s.clone()),
            Self::UserError(s) => Self::UserError(s.clone()),
            _ => Self::ServerError(None),
        }
    }
    pub fn get_sanitized_message(&self) -> String {
        match self {
            Self::RequestValidationError(s) => s.clone().unwrap_or(String::default()),
            Self::JwtCookieNotFoundError(s) => s.clone().unwrap_or(String::default()),
            Self::TamperedJwtTokenError(s) => s.clone().unwrap_or(String::default()),
            Self::UserError(s) => s.clone().unwrap_or(String::default()),
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
            }
            _ => { String::default() }
        }
    }

    pub fn as_numeric(&self) -> u16 {
        match self {
            UserIdentityServiceError::RequestValidationError(_) => 400,
            UserIdentityServiceError::JwtCookieNotFoundError(_) => 404,
            UserIdentityServiceError::TamperedJwtTokenError(_) => 400,
            UserIdentityServiceError::UserError(_) => 400,
            UserIdentityServiceError::ServerError(_) => 500,

            UserIdentityServiceError::ConnectionError(_) => {
                500
            }
            UserIdentityServiceError::ResponseParsingError(_) => {
                500
            },
            _ => {
                500
            },
        }
    }
}
//
// impl From<AdapterError> for AggregatorError {
//     fn from(code: AdapterError) -> Self {
//         match code {
//
//             AdapterError::AmbiguousLocationNameError(candidates) => {
//                 AggregatorError::AmbiguousLocationNameError(candidates)
//             }
//             AdapterError::LocationNotFoundError(s) => {
//                 AggregatorError::LocationNotFoundError(s)
//             }
//             AdapterError::ServerError => {
//                 AggregatorError::ServerError(None)
//             }
//
//         }
//     }
// }
//
// impl From<CacheError> for AggregatorError {
//     fn from(code: CacheError) -> Self {
//         match code {
//             CacheError::CacheMissError(_lat, _lon) => {
//                 AggregatorError::CacheMissError
//             }
//             CacheError::RequestValidationError(s) => {
//                 AggregatorError::RequestParametersValidationError(s)
//             }
//             CacheError::ServerError => {
//                 AggregatorError::ServerError(None)
//             },
//             CacheError::StoringCacheError(s) => {
//                 AggregatorError::StoringCacheError(s)
//             },
//             CacheError::OnlyPotentialMatchesFoundError(candidates) => {
//                 AggregatorError::OnlyPotentialMatchesFoundError(candidates)
//             }
//         }
//     }
// }