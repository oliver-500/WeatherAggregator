use async_trait::async_trait;
use crate::org::unibl::etf::external_dependency_systems::message_broker::broker_error::BrokerError;
// Most common way to handle this currently

#[async_trait]
pub trait MessageConsumer: Send + Sync {
    async fn consume(&self) -> Result<(), BrokerError>;
    fn name(&self) -> &str; // Useful for logging which consumer failed
}