use std::sync::Arc;
use async_trait::async_trait;
use futures_util::StreamExt;
use lapin::options::{BasicAckOptions, BasicConsumeOptions};
use lapin::types::FieldTable;
use crate::org::unibl::etf::external_dependency_systems::message_broker::broker_error::BrokerError;
use crate::org::unibl::etf::external_dependency_systems::message_broker::channel_pool::ChannelPool;
use crate::org::unibl::etf::external_dependency_systems::message_broker::consumers::message_consumer::MessageConsumer;
use crate::org::unibl::etf::model::domain::messages::anonymous_user_registered::AnonymousUserRegistered;
use crate::org::unibl::etf::repositories::user_preferences_repository::UserPreferencesRepository;

pub struct UserIdentityConsumer {
    channel_pool: Arc<ChannelPool>,
    user_preferences_repository: UserPreferencesRepository
}

impl UserIdentityConsumer {

    pub fn new_with_channel_pool_and_user_preferences_repository(
        channel_pool: Arc<ChannelPool>,
        user_preferences_repository: UserPreferencesRepository
    ) -> Self {
        Self {
            channel_pool,
            user_preferences_repository
        }
    }
}

#[async_trait]
impl MessageConsumer for UserIdentityConsumer {
    fn name(&self) -> &str { "UserIdentityConsumer" }

    async fn consume(&self) -> Result<(), BrokerError> {
        let channel = match self.channel_pool.get().await {
            Ok(channel) => channel,
            Err(msg) => {
                return Err(BrokerError::Error("RabbitMQ channel is not available. ".to_string() + msg.as_str()));
            }
        };

        let mut consumer = channel
            .inner
            .basic_consume(
                "user_preferences_service_queue",
                "preferences_service_consumer_tag",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();

        println!("cekam");
        while let Some(delivery_result) = consumer.next().await {
            match delivery_result {

                Ok(delivery) => {
                    // Determine which struct to use based on the routing key
                    println!("op2");
                    match delivery.routing_key.as_str() {
                        "user.registered.anonymous" => {
                            match serde_json::from_slice::<AnonymousUserRegistered>(&delivery.data) {
                                Ok(event) => {
                                    println!("Processing Anonymous: {:?}", event);
                                    // TODO: Add DB logic here
                                }
                                Err(e) => {
                                    tracing::warn!("Failed to parse event. Exact error: {:?}", e.to_string());
                                }
                            }

                        },
                        "user.registered.standard" => {
                            // You'll need a StandardUserRegistered struct too
                            println!("Processing Standard User upgrade");
                        },
                        _ => tracing::warn!("Received unknown routing key: {}", delivery.routing_key),
                    }

                    // 4. Acknowledge the message
                    delivery.ack(BasicAckOptions::default()).await
                        .map_err(|e| tracing::error!("Failed to ack: {}", e)).ok();
                }
                Err(e) => {
                    tracing::error!("Error in consumer stream: {}", e);
                    // Optionally break or continue based on error type
                }
            }
        }
        Ok(())
    }
}