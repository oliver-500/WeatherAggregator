use std::time::{Duration};
use lapin::ConnectionState;
use tokio::sync::{mpsc};
use crate::org::unibl::etf::external_dependency_systems::message_broker::broker_error::BrokerError;
use crate::org::unibl::etf::external_dependency_systems::message_broker::broker_task::BrokerTask;
use crate::org::unibl::etf::external_dependency_systems::message_broker::broker_util;
use crate::org::unibl::etf::external_dependency_systems::message_broker::broker_util::{assure_needed_permissions, configure_channel, schedule_network_task_non_blocking};

pub async fn broker_task_handler(
    mut rx: mpsc::Receiver<BrokerTask>,
    tx: mpsc::Sender<BrokerTask>
) {
    while let Some(task) = rx.recv().await {
        match task {
            BrokerTask::ConnectBroker {
                connection_uri,
                tls_config_source,
                connection,
                is_broker_up,
            } => {
                let sender = tx.clone();
                tokio::spawn(async move {
                    let mut conn_guard = connection.lock().await;

                    if let Some(conn) = conn_guard.as_ref() {
                        if conn.status().state() == ConnectionState::Connected {
                            return;
                        }
                        *conn_guard = None; //dropping old connection
                    };
                    drop(conn_guard);

                    let res = broker_util::connect(
                        connection_uri.clone(),
                        tls_config_source.clone(),
                        {
                            is_broker_up.store(false, std::sync::atomic::Ordering::Relaxed);
                            connection.lock().await.take();

                            let sender = sender.clone();
                            let connection_uri = connection_uri.clone();
                            let tls_config_source = tls_config_source.clone();
                            let connection = connection.clone();
                            let is_broker_up = is_broker_up.clone();

                            move |_error| {
                                broker_util::schedule_network_task_blocking(
                                    sender.clone(),
                                    BrokerTask::ConnectBroker {
                                        connection_uri: connection_uri.clone(),
                                        tls_config_source: tls_config_source.clone(),
                                        connection: connection.clone(),
                                        is_broker_up: is_broker_up.clone()
                                    },
                                )
                            }
                        },
                    ).await;

                    match res {
                        Ok(conn) => {
                            tracing::info!("Connected to broker");
                            let mut conn_guard = connection.lock().await;
                            *conn_guard = Some(conn);
                        }
                        Err(e) => {
                            tracing::info!("Failed to connect to broker: {}", e);

                            let sender_clone = sender.clone();
                            let connection_uri_clone = connection_uri.clone();
                            let tls_config_source_clone = tls_config_source.clone();
                            let connection_clone = connection.clone();

                            tracing::info!("Trying to connect to broker in 1000ms again");
                            tokio::time::sleep(Duration::from_millis(1000)).await;

                            schedule_network_task_non_blocking(
                                sender_clone,
                                BrokerTask::ConnectBroker {
                                    connection_uri: connection_uri_clone.clone(),
                                    tls_config_source: tls_config_source_clone.clone(),
                                    connection: connection_clone.clone(),
                                    is_broker_up: is_broker_up.clone()
                                }
                            ).await;

                        }
                    }
                });
            }
            BrokerTask::CreateChannelToBroker {
                connection,
                channel_tx,
            } => {
                let sender = tx.clone();

                tokio::spawn(async move {

                    let connection_pointer = connection.clone();
                    let sender_clone = sender.clone();
                    let channel_tx_clone = channel_tx.clone();

                    let res = broker_util::create_channel(
                        connection.clone(),
                        move |_error| {
                            broker_util::schedule_network_task_blocking(
                                sender_clone.clone(),
                                BrokerTask::CreateChannelToBroker {
                                    connection: connection_pointer.clone(),
                                    channel_tx: channel_tx_clone.clone(),
                                }
                            )
                        }
                    ).await;


                    match res {
                        Ok(channel) => {
                            tracing::info!("Successfully created a channel");
                            channel_tx.send(channel).await.unwrap();
                            tracing::info!("Successfully created a channel ------------ 2");
                        },
                        Err(e) => {
                            tracing::info!("Failed to create a channel: {}", e);
                            tracing::info!("Trying to recreate channel to the broker in 1000ms");
                            tokio::time::sleep(Duration::from_millis(1000)).await;
                            match e {
                                BrokerError::ChannelCreationError(_e1) => {
                                    let sender_clone = sender.clone();
                                    let connection_clone = connection.clone();

                                    schedule_network_task_non_blocking(
                                        sender_clone.clone(),
                                        BrokerTask::CreateChannelToBroker {
                                            connection: connection_clone.clone(),
                                            channel_tx,
                                        }
                                    ).await;
                                },
                                _ => {
                                    return;
                                }
                            };
                        }
                    }
                });
            }
            BrokerTask::ConfigureChannelToBroker {
                channel,
            } => {
                let sender = tx.clone();

                tokio::spawn(async move{
                    let res= configure_channel(
                        channel.clone(),
                    ).await;

                    match res {
                        Ok(_) => {
                            tracing::info!("Successfully reconfigured a channel");
                            // let sender_clone = sender.clone();
                            // let channel_clone = channel.clone();
                            //
                            // schedule_network_task_non_blocking(
                            //     sender_clone,
                            //     BrokerTask::AssureNeededPermissionsToBroker {
                            //         channel: channel_clone.clone(),
                            //     }
                            // ).await;

                        },
                        Err(e) => {
                            tracing::info!("Failed to configure channel: {}", e);

                            let channel_clone = channel.clone();
                            let sender_clone = sender.clone();

                            tracing::info!("Trying to reconfigure channel to the broker in 1000ms");
                            tokio::time::sleep(Duration::from_millis(1000)).await;

                            match e {
                                BrokerError::ChannelConfigurationError(_) => {
                                    schedule_network_task_non_blocking(
                                        sender_clone.clone(),
                                        BrokerTask::ConfigureChannelToBroker {
                                            channel: channel_clone.clone(),
                                        }
                                    ).await;
                                },
                                _ => {}
                            }
                        }
                    }
                });
            }
            BrokerTask::AssureNeededPermissionsToBroker {
                channel,
            } => {
                let sender = tx.clone();

                tokio::spawn(async move {
                    let channel_clone = channel.clone();

                    let res= assure_needed_permissions(
                        channel_clone.clone(),
                    ).await;

                    match res {
                        Ok(_) => {
                            tracing::info!("Successfully assured needed permissions");
                        },
                        Err(e) => {
                            tracing::info!("Failed to assure needed permissions: {}", e);

                            match e {
                                BrokerError::Error(_) => {
                                    let sender_clone = sender.clone();
                                    let channel_clone = channel.clone();

                                    tracing::info!("Trying to reassure needed permissions to the broker in 1000ms");
                                    tokio::time::sleep(Duration::from_millis(1000)).await;

                                    schedule_network_task_non_blocking(
                                        sender_clone.clone(),
                                        BrokerTask::AssureNeededPermissionsToBroker {
                                            channel: channel_clone,
                                        }
                                    ).await;
                                },
                                _ => {},
                            }
                        }
                    }
                });
            },
            _ => {}
        }
    }
}