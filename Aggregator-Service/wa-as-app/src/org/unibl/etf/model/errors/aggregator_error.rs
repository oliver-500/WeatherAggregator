use serde::{Deserialize, Serialize};
use crate::org::unibl::etf::model::errors::external_api_adapter_error_message::{AdapterError, LocationCandidate};

#[derive(Serialize, Debug, Clone, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AggregatorError {
    AmbiguousLocation(Vec<LocationCandidate>), // Keep this specific for the UI
    ConnectionError(String),
    ValidationError(String),
    ServerError(Option<String>),
    ResponseParsingError(String),

    // The Catch-all: This maps things like EndpointNotFound or UnknownProvider
    #[serde(rename = "UNEXPECTED_ADAPTER_ERROR")]
    UnexpectedError(String),
}

impl AggregatorError {
    pub fn get_message(&self) -> String {
        match self {
            AggregatorError::ConnectionError(msg) => msg.clone(),
            AggregatorError::ValidationError(msg) => msg.clone(),
            AggregatorError::ServerError(msg) => msg.clone().unwrap_or("".to_owned()),
            _ => { "".to_string() }
        }
    }

    pub fn as_numeric(&self) -> u16 {
        match self {
            AggregatorError::ConnectionError(_) => 500,
            AggregatorError::ValidationError(_) => 400,
            AggregatorError::ServerError(_) => 500,
            AggregatorError::ResponseParsingError(_) => 500,
            AggregatorError::AmbiguousLocation(_) => 400,
            AggregatorError::UnexpectedError(_) => 500,
        }
    }
}

impl From<AdapterError> for AggregatorError {
    fn from(code: AdapterError) -> Self {
        match code {
            // 1. Direct Mappings
            AdapterError::AmbiguousLocation(c) => AggregatorError::AmbiguousLocation(c),
            AdapterError::ConnectionError(s) => AggregatorError::ConnectionError(s),
            AdapterError::ServerError(s) => AggregatorError::ServerError(s),

            // 2. Grouped into ValidationError (Client's fault)
            AdapterError::ValidationError |
            AdapterError::LocationNotFound(_) |
            AdapterError::EndpointNotFound => {
                AggregatorError::ValidationError("Request parameters are invalid or location not found".into())
            },

            // 3. Grouped into ResponseParsingError (Adapter/Server fault)
            AdapterError::ApiResponseParsingError(s) |
            AdapterError::InvalidProviderResponse(s) => {
                AggregatorError::ResponseParsingError(s)
            },

            // 4. Everything else goes to the Generic variant
            _ => AggregatorError::UnexpectedError("An unhandled provider error occurred".into()),
        }
    }
}