use std::sync::Arc;
use async_trait::async_trait;
use futures_util::StreamExt;
use lapin::options::{BasicAckOptions, BasicConsumeOptions};
use lapin::types::FieldTable;
use crate::org::unibl::etf::external_dependency_systems::message_broker::broker_error::BrokerError;
use crate::org::unibl::etf::external_dependency_systems::message_broker::channel_pool::ChannelPool;
use crate::org::unibl::etf::external_dependency_systems::message_broker::consumers::message_consumer::MessageConsumer;
use crate::org::unibl::etf::model::domain::entities::user_preferences_entity::unit_system_type::UnitSystemType;
use crate::org::unibl::etf::model::domain::entities::user_preferences_entity::UserPreferencesEntity;
use crate::org::unibl::etf::model::domain::messages::anonymous_user_registered::AnonymousUserRegistered;
use crate::org::unibl::etf::model::domain::messages::standard_user_registered::StandardUserRegistered;

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

                                    let user_preferences_entity = UserPreferencesEntity {
                                        user_id: event.id,
                                        user_type: Some(event.user_type),
                                        unit_system: UnitSystemType::METRIC,
                                        favorite_location_name: None,
                                        favorite_lat: None,
                                        favorite_lon: None,
                                        updated_at: Default::default(),
                                    };

                                    match self.user_preferences_repository.save(&user_preferences_entity)
                                        .await {
                                        Ok(_) => {
                                            tracing::info!("Successfully saved user preferences for anonymous user.");
                                        },
                                        Err(e) => {
                                            tracing::error!("Error saving user preferences entity: {:?}", e.to_string());
                                        }
                                    }

                                }
                                Err(e) => {
                                    tracing::warn!("Failed to parse event. Exact error: {:?}", e.to_string());
                                }
                            }

                        },
                        "user.registered.standard" => {
                            match serde_json::from_slice::<StandardUserRegistered>(&delivery.data) {
                                Ok(event) => {
                                    println!("Processing Standard User upgrade");
                                    // TODO: Add DB logic here

                                    match event.old_id {
                                        Some(old_id) => {
                                            match self.user_preferences_repository
                                                .migrate_guest_to_user(old_id, event.new_id)
                                                .await {
                                                Ok(_) => {
                                                    tracing::info!("Successfully migrated user from anonymous to standard.");
                                                },
                                                Err(e) => {
                                                    tracing::error!("Failed to migrate user from anonymous to standard: {}", e.to_string());
                                                }
                                            }
                                        },
                                        None => {
                                            let user_preferences_entity = UserPreferencesEntity {
                                                user_id: event.new_id,
                                                user_type: Some(event.user_type),
                                                unit_system: UnitSystemType::METRIC,
                                                favorite_location_name: None,
                                                favorite_lat: None,
                                                favorite_lon: None,
                                                updated_at: Default::default(),
                                            };

                                            match self.user_preferences_repository.save(&user_preferences_entity)
                                                .await {
                                                Ok(_) => {
                                                    tracing::info!("Successfully saved user preferences for anonymous user.");
                                                },
                                                Err(e) => {
                                                    tracing::error!("Error saving user preferences entity: {:?}", e.to_string());
                                                }
                                            }
                                        }
                                    }



                                }
                                Err(e) => {
                                    tracing::warn!("Failed to parse event. Exact error: {:?}", e.to_string());
                                }
                            }

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