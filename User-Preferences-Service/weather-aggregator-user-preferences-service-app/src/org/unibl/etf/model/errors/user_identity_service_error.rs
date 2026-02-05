use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserPreferencesServiceError {
    RequestValidationError(Option<String>),
    ServerError(Option<String>),
    ConnectionError(Option<String>),
    ResponseParsingError(Option<String>),
    JwtCookieNotFoundError(Option<String>),
    TamperedJwtTokenError(Option<String>),
    DatabaseError(Option<String>),
    UserError(Option<String>),
    Unauthorized(Option<String>),
    ApplicationCurrentlyUnavailable,
    HistoryItemAlreadyExistsError(Option<String>),

}

impl UserPreferencesServiceError {
    pub fn get_sanitized_error(&self) -> Self {
        match self {
            Self::RequestValidationError(s) => Self::RequestValidationError(s.clone()),
            Self::JwtCookieNotFoundError(s) => Self::JwtCookieNotFoundError(s.clone()),
            Self::TamperedJwtTokenError(s) => Self::TamperedJwtTokenError(s.clone()),
            Self::Unauthorized(s) => Self::Unauthorized(s.clone()),
            Self::UserError(s) => Self::UserError(s.clone()),
            Self::ApplicationCurrentlyUnavailable => Self::ApplicationCurrentlyUnavailable,
            Self::HistoryItemAlreadyExistsError(s) => Self::HistoryItemAlreadyExistsError(s.clone()),
            _ => Self::ServerError(None),
        }
    }
    pub fn get_sanitized_message(&self) -> String {
        match self {
            Self::RequestValidationError(s) => s.clone().unwrap_or(String::default()),
            Self::JwtCookieNotFoundError(s) => s.clone().unwrap_or(String::default()),
            Self::TamperedJwtTokenError(s) => s.clone().unwrap_or(String::default()),
            Self::UserError(s) => s.clone().unwrap_or(String::default()),
            Self::Unauthorized(s) => s.clone().unwrap_or(String::default()),
            Self::ApplicationCurrentlyUnavailable => "Application currently unavailable.".to_string(),
            Self::HistoryItemAlreadyExistsError(s) => s.clone().unwrap_or(String::default()),
            _ => String::default(),
        }

    }

    pub fn get_message(&self) -> String {
        match self {
            UserPreferencesServiceError::RequestValidationError(msg) => {
                msg.clone().unwrap_or(String::from("RequestValidationError"))
            },
            UserPreferencesServiceError::JwtCookieNotFoundError(msg) => {
                msg.clone().unwrap_or(String::from("JwtCookieNotFoundError"))
            },
            UserPreferencesServiceError::TamperedJwtTokenError(msg) => {
                msg.clone().unwrap_or(String::from("TamperedJwtTokenError"))
            },
            UserPreferencesServiceError::UserError(msg) => {
                msg.clone().unwrap_or(String::from("UserError"))
            },
            UserPreferencesServiceError::Unauthorized(msg) => {
                msg.clone().unwrap_or(String::from("Unauthorized"))
            },
            UserPreferencesServiceError::ApplicationCurrentlyUnavailable => {
                String::from("Application currently unavailable.")
            }
            _ => { String::default() }
        }
    }

    pub fn as_numeric(&self) -> u16 {
        match self {
            UserPreferencesServiceError::RequestValidationError(_) => 400,
            UserPreferencesServiceError::JwtCookieNotFoundError(_) => 404,
            UserPreferencesServiceError::TamperedJwtTokenError(_) => 400,
            UserPreferencesServiceError::UserError(_) => 400,
            UserPreferencesServiceError::Unauthorized(_) => 401,
            UserPreferencesServiceError::ServerError(_) => 500,
            UserPreferencesServiceError::HistoryItemAlreadyExistsError(_) => 409,

            UserPreferencesServiceError::ConnectionError(_) => {
                500
            }
            UserPreferencesServiceError::ResponseParsingError(_) => {
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