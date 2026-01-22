use std::sync::Arc;
use async_trait::async_trait;
use futures_util::StreamExt;
use lapin::options::{BasicAckOptions, BasicConsumeOptions};
use lapin::types::FieldTable;
use opentelemetry::global;
use opentelemetry::propagation::Extractor;
use tracing_opentelemetry::OpenTelemetrySpanExt;
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

struct RabbitMqHeaderExtractor<'a>(Option<&'a lapin::types::FieldTable>);

impl<'a> Extractor for RabbitMqHeaderExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        let value = self.0?.inner().get(key)?;

        // Check both ShortString and LongString variants
        match value {
            lapin::types::AMQPValue::ShortString(s) => Some(s.as_str()),
            lapin::types::AMQPValue::LongString(s) => {
                // s.as_bytes() returns &[u8]
                // We convert that to &str
                std::str::from_utf8(s.as_bytes()).ok()
            },
            _ => None,
        }
    }

    fn keys(&self) -> Vec<&str> {
        match self.0 {
            Some(table) => table
                .inner()
                .keys()
                .map(|k| k.as_str())
                .collect(),
            None => Vec::new(),
        }
    }
}

#[async_trait]
impl MessageConsumer for UserIdentityConsumer {
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

        while let Some(delivery_result) = consumer.next().await {
            let Ok(delivery) = delivery_result else {
                tracing::error!("Error in consumer stream: {:?}", delivery_result.err());
                continue;
            };

            let headers = delivery.properties.headers().as_ref();

            println!("www{:?}", headers);

            // 2. Extract the remote parent context (Trace ID / Span ID) from headers
            // We treat the RabbitMQ headers as a map for OpenTelemetry to read
            let parent_cx = global::get_text_map_propagator(|prop| {
                // You may need a wrapper to convert RabbitMQ FieldTable to a HeaderExtractor
                prop.extract(&RabbitMqHeaderExtractor(headers))
            });

            // 3. Create a new span that is "linked" to the producer's span
            let span = tracing::info_span!("consume_message");
            span.set_parent(parent_cx);
            let _guard = span.enter();

            match delivery.routing_key.as_str() {
                "user.registered.anonymous" => {
                    let parsing_result = serde_json::from_slice::<AnonymousUserRegistered>(&delivery.data);
                    let Ok(event) = parsing_result else {
                        tracing::warn!("Failed to parse event with error: {:?}", parsing_result.err());
                        continue;
                    };

                    println!("Processing Anonymous: {:?}", event);
                    let user_preferences_entity = UserPreferencesEntity {
                        user_id: event.id,
                        user_type: Some(event.user_type),
                        unit_system: UnitSystemType::METRIC,
                        favorite_location_name: None,
                        favorite_lat: None,
                        favorite_lon: None,
                        updated_at: Default::default(),
                    };

                    match self.user_preferences_repository
                        .save(&user_preferences_entity)
                        .await {
                            Ok(_) => {
                                tracing::info!("Successfully saved user preferences for anonymous user.");
                            },
                            Err(e) => {
                                tracing::error!("Error saving user preferences entity: {:?}", e.to_string());
                            }
                    }
                },
                "user.registered.standard" => {
                    let parsing_result = serde_json::from_slice::<StandardUserRegistered>(&delivery.data);
                    let Ok(event) = parsing_result else {
                        tracing::warn!("Failed to parse event with error: {:?}", parsing_result.err());
                        continue;
                    };

                    println!("Processing Standard User upgrade");
                    if let Some(old_id) = event.old_id {
                        match self.user_preferences_repository
                            .migrate_guest_to_user(old_id, event.new_id)
                            .await {
                            Ok(_) => {
                                tracing::info!("Successfully migrated user from anonymous to standard.");
                            },
                            Err(e) => {
                                match e {
                                    sqlx::Error::RowNotFound => {
                                        tracing::info!("Guest id with that id not found. Saving as a new user.")
                                    },
                                    _ => {
                                        tracing::error!("Failed to migrate user from anonymous to standard: {}", e.to_string());
                                        continue;
                                    }
                                }
                            }
                        }
                    }

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
                },
                _ => tracing::warn!("Received unknown routing key: {}", delivery.routing_key),
            }

            // 4. Acknowledge the message
            delivery.ack(BasicAckOptions::default()).await
                .map_err(|e| tracing::error!("Failed to ack: {}", e)).ok();
        }

        Ok(())
    }

    fn name(&self) -> &str { "UserIdentityConsumer" }
}