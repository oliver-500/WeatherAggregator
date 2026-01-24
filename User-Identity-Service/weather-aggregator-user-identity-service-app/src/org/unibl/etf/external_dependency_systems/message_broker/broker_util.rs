use std::sync::{Arc};
use lapin::{BasicProperties, Channel, ChannelState, Connection, ConnectionProperties, ConnectionState};
use lapin::options::{BasicPublishOptions, ExchangeDeclareOptions};
use lapin::publisher_confirm::Confirmation;
use lapin::tcp::{OwnedIdentity, OwnedTLSConfig};
use lapin::types::FieldTable;

use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex};

use crate::org::unibl::etf::external_dependency_systems::message_broker::broker_error::BrokerError;
use crate::org::unibl::etf::external_dependency_systems::message_broker::broker_task::BrokerTask;
use crate::org::unibl::etf::external_dependency_systems::message_broker::channel_pool::PreparingChannel;


pub async fn connect<F>(
    connection_uri: String,
    tls_config_source: Arc<Option<TLSConfigDataSource>>,
    on_error: F
) -> Result<Connection, BrokerError>
where
    F: Fn(lapin::Error) + Send + Sync + 'static,
{
    let conn;

    if let Some(tls_config_source) = &*tls_config_source {
        conn = Some(Connection::connect_with_config(
            connection_uri.as_str(),
            ConnectionProperties::default(),
            tls_config_source.build_tls_config(),
        ).await.map_err(|e1| {
            return BrokerError::ConnectionError(e1.to_string());
        })?);
    }
    else {
        conn = Some(Connection::connect(
            connection_uri.as_str(),
            ConnectionProperties::default(),
        ).await.map_err(|e1| {
            return BrokerError::ConnectionError(e1.to_string());
        })?);
    }

    let connection = conn.unwrap();

    connection.on_error(move |error| {
        tracing::error!("Broker connection error occurred: {}", error.to_string());
        on_error(error);
    });

    Ok(connection)
}

pub async fn create_channel<F>(
    connection_pointer: Arc<Mutex<Option<Connection>>>,
    on_error: F,
) -> Result<Channel, BrokerError> where
    F: Fn(lapin::Error) + Send + Sync + 'static,
{
    let conn_guard = connection_pointer.lock().await;
    let conn = conn_guard.as_ref();

    let conn = match conn {
        Some(conn) => {
            if conn.status().state() == ConnectionState::Connected {
                conn
            } else {
                return Err(BrokerError::ConnectionError("Not connected to create a channel".to_string()));
            }
        }
        None => {
            return Err(BrokerError::ConnectionError("Connection not present to create a channel".to_string()));
        }
    };

    let channel = conn.create_channel().await.map_err(|e| {
        return BrokerError::ChannelCreationError(e.to_string());
    })?;
    drop(conn_guard);

    channel.on_error(move |error: lapin::Error| {
        tracing::error!("Broker channel error occurred: {}", error.to_string());
        on_error(error);
    });

    Ok(channel)
}

pub async fn configure_channel(
    channel_pointer: Arc<Mutex<Option<PreparingChannel>>>,
) -> Result<(), BrokerError> {
    let mut channel_guard = channel_pointer.lock().await;

    let preparing_channel = match channel_guard.as_mut() {
        Some(channel) => {
            channel
        },
        None => {
            return Err(BrokerError::Error("No channel to configure.".to_string()));
        }
    };

    let channel = match &preparing_channel.inner {
        None => {
            return Err(BrokerError::Error("Channel pointer empty.".to_string()))
        }
        Some(channel) => {
            if channel.status().state() != ChannelState::Connected {
                return Err(BrokerError::Error("Channel not connected to configure it".to_string()));
            }
            channel
        }
    };

    match channel
        .exchange_declare(
            "user_events",
            lapin::ExchangeKind::Topic,
            ExchangeDeclareOptions {
                durable: true, // Must be true!
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await {
        Ok(_) => {
            preparing_channel.configured = true;
            Ok(())
        },
        Err(error) => {
            Err(BrokerError::ChannelConfigurationError(error.to_string()))
        }
    }
}

pub async fn assure_needed_permissions(
    channel_pointer: Arc<Mutex<Option<PreparingChannel>>>,
) -> Result<(), BrokerError> {

    let mut channel_guard = channel_pointer.lock().await;

    let preparing_channel = match channel_guard.as_mut() {
        Some(channel) => {
            channel
        },
        None => {
            return Err(BrokerError::Error("No channel to check permissions for.".to_string()));
        }
    };

    let channel = match &preparing_channel.inner {
        None => {
            return Err(BrokerError::Error("Channel pointer empty.".to_string()))
        }
        Some(channel) => {
            if channel.status().state() != ChannelState::Connected {
                return Err(BrokerError::ChannelConfigurationError("Channel not connected to check permissions for".to_string()));
            }
            channel
        }
    };

    let confirm = match channel
        .basic_publish(
            "user_events",
            "dummy_key",
            BasicPublishOptions::default(),
            &*b"probe".to_vec(),
            BasicProperties::default()
        )
    .await {
        Ok(c) => {
            tracing::info!("Dummy key publish successfully");
            preparing_channel.permissions_checked = true;
            c
        }
        Err(e) => {
            return Err(BrokerError::PublishingError(e.to_string()));
        }
    }.await;

    match confirm {
        Ok(Confirmation::NotRequested) => tracing::info!("Confirmation not requested."),
        Ok(Confirmation::Ack(_)) => tracing::info!("Broker accepted"),
        Ok(Confirmation::Nack(_)) => tracing::info!("Broker rejected"),
        Err(e) => {
            return Err(BrokerError::PublishingError(e.to_string()));
        },
    }

    Ok(())
}

pub fn schedule_network_task_blocking(
    tx: Sender<BrokerTask>,
    task: BrokerTask
) {
    match tx.blocking_send(task) {
        Ok(_) => {
            tracing::trace!("Sent a retry task over mpsc channel");
        }
        Err(error) => {
            tracing::error!("Could not send retry task: {}", error);
        }
    }
}

pub async fn schedule_network_task_non_blocking(
    tx: Sender<BrokerTask>,
    task: BrokerTask
) {
    match tx.send(task).await {
        Ok(_) => {
            tracing::trace!("Sent a retry task over mpsc channel");
        }
        Err(error) => {
            tracing::error!("Could not send retry task: {}", error);
        }
    }
}


#[derive(Clone)]
pub struct TLSConfigDataSource {
    pub identity: Option<TlsOwnedIdentityPKCS12>,
    pub cert_chain: Option<String>,
}

impl TLSConfigDataSource {
    pub fn build_tls_config(&self) -> OwnedTLSConfig {

        let identity = match &self.identity {
            None => {
                None
            }
            Some(_identity) => {
                Some(OwnedIdentity::PKCS12 {
                    der: self.identity.clone().unwrap().der.clone() ,
                    password: self.identity.clone().unwrap().password.clone() ,
                })
            }
        };

        let tls_config = OwnedTLSConfig {
            identity,
            cert_chain: self.cert_chain.clone(),
        };

        tls_config
    }
}

#[derive(Clone)]
pub struct TlsOwnedIdentityPKCS12 {
    pub der: Vec<u8>,
    pub password: String,
}


