use std::sync::Arc;
use lapin::BasicProperties;
use lapin::options::BasicPublishOptions;
use lapin::types::FieldTable;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use crate::org::unibl::etf::external_dependency_systems::message_broker::broker_error::BrokerError;
use crate::org::unibl::etf::external_dependency_systems::message_broker::channel_pool::ChannelPool;
use crate::org::unibl::etf::model::domain::messages::anonymous_user_registered::AnonymousUserRegistered;
use crate::org::unibl::etf::model::domain::messages::standard_user_registered::StandardUserRegistered;

#[derive(Debug, Clone)]
pub struct UserPublisher {
    pub broker_pool : Arc<ChannelPool>
}

use opentelemetry::propagation::Injector;
use lapin::types::{ShortString, AMQPValue};

struct RabbitMqHeaderInjector<'a>(&'a mut FieldTable);

impl<'a> Injector for RabbitMqHeaderInjector<'a> {
    fn set(&mut self, key: &str, value: String) {
        self.0.insert(
            ShortString::from(key),
            AMQPValue::LongString(value.into()),
        );
    }
}

impl UserPublisher {
    pub fn new_with_channel_pool(broker_pool: Arc<ChannelPool>) -> Self {
        Self {
            broker_pool
        }
    }

    #[tracing::instrument(name = "Publish user Registered event", skip(self)) ]
    pub async fn publish_anonymous_user_registered_event(
        &self,
        event: AnonymousUserRegistered
    ) -> Result<(), BrokerError> {

        let payload = serde_json::to_vec(&event).map_err(|e| {
            BrokerError::Error(format!("Payload not serializable to json. Error: {}", e.to_string()))
        })?;

        let channel = match self.broker_pool.get().await  {
            Err(msg) => {
                tracing::error!("RabbitMQ channel is not available");
                return Err(BrokerError::Error("RabbitMQ channel is not available. ".to_string() + msg.as_str()));
            }
            Ok(c) => {
                c
            },
        };

        let mut headers = FieldTable::default();

        // 1. Get the current tracing context (the active Span)
        let context = tracing::Span::current().context();

        // 2. Inject that context into the RabbitMQ headers
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut RabbitMqHeaderInjector(&mut headers));
        });

        let publisher_confirm = match channel
            .inner
            .basic_publish(
                "user_events",
                "user.registered.anonymous",
                BasicPublishOptions::default(),
                &*payload,
                BasicProperties::default()
                    .with_delivery_mode(2) // Persistent message
                    .with_content_type("application/json".into())
                    .with_headers(headers),
            )
            .await {
            Ok(r) => {
                tracing::info!("Message sent successfully");
                r
            },
            Err(e) => {
                tracing::error!("Message could not be sent to broker: {}", e.to_string());
                return Err(BrokerError::PublishingError("Message could not be sent to broker".to_string()));
            }
        };

        let _con = match publisher_confirm
            .await {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("Did not receive confirmation from broker: {}", e.to_string());
                return Err(BrokerError::ConfirmationError("Did not receive confirmation from broker".to_string()));
            }
        };

        Ok(())
    }


    #[tracing::instrument(name = "Publish user Registered event", skip(self)) ]
    pub async fn publish_standard_user_registered_event(
        &self,
        event: StandardUserRegistered
    ) -> Result<(), BrokerError> {

        let payload = serde_json::to_vec(&event).map_err(|e| {
            BrokerError::Error(format!("Payload not serializable to json. Error: {}", e.to_string()))
        })?;

        let channel = match self.broker_pool.get().await  {
            Err(msg) => {
                tracing::error!("RabbitMQ channel is not available");
                return Err(BrokerError::Error("RabbitMQ channel is not available. ".to_string() + msg.as_str()));
            }
            Ok(c) => {
                c
            },
        };

        let mut headers = FieldTable::default();

        // 1. Get the current tracing context (the active Span)
        let context = tracing::Span::current().context();

        // 2. Inject that context into the RabbitMQ headers
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut RabbitMqHeaderInjector(&mut headers));
        });

        let publisher_confirm = match channel
            .inner
            .basic_publish(
                "user_events",
                "user.registered.standard",
                BasicPublishOptions::default(),
                &*payload,
                BasicProperties::default()
                    .with_delivery_mode(2) // Persistent message
                    .with_content_type("application/json".into())
                    .with_headers(headers),
            )
            .await {
            Ok(r) => {
                tracing::info!("Message sent successfully");
                r
            },
            Err(e) => {
                tracing::error!("Message could not be sent to broker: {}", e.to_string());
                return Err(BrokerError::PublishingError("Message could not be sent to broker".to_string()));
            }
        };

        let _con = match publisher_confirm
            .await {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("Did not receive confirmation from broker: {}", e.to_string());
                return Err(BrokerError::ConfirmationError("Did not receive confirmation from broker".to_string()));
            }
        };

        Ok(())
    }


}

// impl Default for UserPublisher {
//     fn default() -> Self {
//         Self::new()
//     }
// }