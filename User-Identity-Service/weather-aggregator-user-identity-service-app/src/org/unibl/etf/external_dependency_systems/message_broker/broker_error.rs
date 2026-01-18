use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrokerError {
    #[error("Connection Error: {0}")]
    ConnectionError(String),
    #[error("Channel Error: {0}")]
    ChannelCreationError(String),
    #[error("Channel Configuration Error: {0}")]
    ChannelConfigurationError(String),
    #[error("Publishing Error: {0}")]
    PublishingError(String),
    #[error("{0}")]
    Error(String),
    #[error("{0}")]
    ConfirmationError(String),
}