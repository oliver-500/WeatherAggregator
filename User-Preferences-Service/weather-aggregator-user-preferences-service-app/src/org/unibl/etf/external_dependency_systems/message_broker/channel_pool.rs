use lapin::{Channel, Connection};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::{Duration};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, Mutex};
use crate::org::unibl::etf::external_dependency_systems::message_broker::broker_task::BrokerTask;
use crate::org::unibl::etf::external_dependency_systems::message_broker::broker_util::{schedule_network_task_non_blocking, TLSConfigDataSource};
use crate::org::unibl::etf::external_dependency_systems::message_broker::consumers::message_consumer::MessageConsumer;

#[derive(Debug, Clone)]
pub struct ChannelPool {
    //inner immutable implementation
    inner: Arc<ChannelPoolInner>,
}

//channel to be prepared for use
pub struct PreparingChannel {
    pub inner: Option<Channel>,
    pub configured: bool,
    pub permissions_checked: bool,
}

//channel in use
pub struct PooledChannel {
    pub inner: Channel,
    sender: Sender<Channel>,
}

#[derive(Debug)]
struct ChannelPoolInner {
    connection: Arc<Mutex<Option<Connection>>>,
    sender: Sender<Channel>,
    receiver: Arc<Mutex<Receiver<Channel>>>,
    size: usize,
    is_emptying: AtomicBool,
}

impl ChannelPool {
    //function to empty channel pool when its connection is dropped
    pub async fn empty_pool(&self, receiver: Arc<Mutex<Receiver<Channel>>>) {
        let mut number_of_channels_consumed = 0;
        self.inner.is_emptying.store(true, std::sync::atomic::Ordering::Relaxed);
        while let Some(_channel) = receiver.lock().await.recv().await {
            number_of_channels_consumed += 1;
            if number_of_channels_consumed == self.inner.size {
                break;
            }
        }
        self.inner.is_emptying.store(false, std::sync::atomic::Ordering::Relaxed);
    }

    //function to replenish channel pool by sending CreateChannelToBroker task to network task handler
    pub async fn replenish_pool(&self, retry_tx: Sender<BrokerTask>) {
        for _ in 0..self.inner.size {
            let retry_task = self.send_create_channel_task().await;
            schedule_network_task_non_blocking(retry_tx.clone(), retry_task).await;
        }
    }

    pub async fn send_create_channel_task(&self) -> BrokerTask {
        BrokerTask::CreateChannelToBroker {
            connection: self.inner.connection.clone(),
            channel_tx: self.inner.sender.clone(),
        }
    }

    pub async fn wait_for_connection_establishment(&self) {
        loop {
            tracing::info!("Waiting for connection establishment between broker and app for 500ms");
            tokio::time::sleep(Duration::from_millis(500)).await;

            let guard = self.inner.connection.lock().await;
            if let Some(_conn) = guard.as_ref() {
                break;
            }
        }
    }

    //function to prepare channels for communication accomplished through configuring it and checking permissions of one channel
    pub async fn prepare_channel(&self, retry_tx: Sender<BrokerTask>, is_broker_up: Arc<AtomicBool>) {
        let mut guard =  self.inner.receiver.lock().await;
        let channel = (*guard).recv().await.unwrap();
        drop(guard);

        let retry_task = self.send_create_channel_task().await;
        schedule_network_task_non_blocking(retry_tx.clone(), retry_task).await;

        let preparing_channel = PreparingChannel {
            inner: Some(channel),
            configured: false,
            permissions_checked: false,
        };

        let channel_pointer = Arc::new(Mutex::new(Some(preparing_channel)));
        let retry_task = BrokerTask::ConfigureChannelToBroker {
            channel: channel_pointer.clone(),
        };
        schedule_network_task_non_blocking(retry_tx.clone(), retry_task).await;

        loop {
            //tracing::trace!("Waiting for channel to be configured for 100ms");
            let conn_guard = self.inner.connection.lock().await;

            //u medjuvremenu se ugasio
            if let None = conn_guard.as_ref() {

                return;
            }
            drop(conn_guard);
            tokio::time::sleep(Duration::from_millis(100)).await;

            let mut guard = channel_pointer.lock().await;
            if let Some(channel) = guard.take() {
                if channel.configured == true {
                    *guard = Some(channel);
                    break;
                }
                else {
                    *guard = Some(channel);
                }
            }
        }

        let retry_task = BrokerTask::AssureNeededPermissionsToBroker {
            channel: channel_pointer.clone(),
        };
        schedule_network_task_non_blocking(retry_tx.clone(), retry_task).await;

        loop {
            let conn_guard = self.inner.connection.lock().await;
            tracing::info!("Waiting for channel to be checked for permissions for 100ms");
            if let None = conn_guard.as_ref() {
                return;
            }
            drop(conn_guard);
            tokio::time::sleep(Duration::from_millis(100)).await;

            let mut guard = channel_pointer.lock().await;
            if let Some(channel) = guard.take() {
                if channel.permissions_checked == true {
                    break;
                }
                else {
                    *guard = Some(channel);
                    continue;
                }
            }
        }
        is_broker_up.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub async fn monitor_connection(
        &self,
        retry_tx: Sender<BrokerTask>,
        is_broker_up: Arc<AtomicBool>,
        consumers: Arc<Vec<Arc<dyn MessageConsumer>>>
    ) {
        loop {

            if is_broker_up.load(std::sync::atomic::Ordering::Relaxed) {
                tokio::time::sleep(Duration::from_millis(1000)).await;
                tracing::trace!("Monitoring connection to broker.");
                continue;
            }
            self.empty_pool(self.inner.receiver.clone()).await;
            self.wait_for_connection_establishment().await;
            self.replenish_pool(retry_tx.clone()).await;
            self.prepare_channel(retry_tx.clone(), is_broker_up.clone()).await;



            for consumer in consumers.iter() {
                let consumer_clone = consumer.clone();
                tokio::spawn(async move {
                    let _ = consumer_clone.consume().await;
                });
            }
        }
    }

    pub async fn    new(
        size: usize
    ) -> Result<Self, String> {
        let connection_pointer = Arc::new(Mutex::new(None));

        let (tx, rx) = mpsc::channel::<Channel>(size);
        Ok(Self {
            inner: Arc::new(ChannelPoolInner {
                connection: connection_pointer.clone(),
                sender: tx.clone(),
                receiver: Arc::new(Mutex::new(rx)),
                size,
                is_emptying: AtomicBool::new(false),
            }),
        })
    }

    pub async fn configure_channel_pool(
        &self,
        broker_connection_uri: String,
        tls_config_source: Option<TLSConfigDataSource>,
        retry_tx: Sender<BrokerTask>,
        is_broker_up: Arc<AtomicBool>,
        consumers: Arc<Vec<Arc<dyn MessageConsumer>>>,
    ) {
        let network_task = BrokerTask::ConnectBroker {
            connection_uri: broker_connection_uri,
            tls_config_source: Arc::new(tls_config_source),
            connection: self.inner.connection.clone(),
            is_broker_up: is_broker_up.clone()
        };
        schedule_network_task_non_blocking(retry_tx.clone(), network_task).await;

        self.wait_for_connection_establishment().await;
        self.replenish_pool(retry_tx.clone()).await;
        self.prepare_channel(retry_tx.clone(), is_broker_up.clone()).await;


        let monitor_clone = self.clone();
        let retry_tx_pointer = retry_tx.clone();
        let is_broker_up_pointer = is_broker_up.clone();

        tokio::spawn(async move {
            monitor_clone.monitor_connection(retry_tx_pointer, is_broker_up_pointer.clone(), consumers).await;
        });
    }

    pub async fn get(&self) -> Result<PooledChannel, String> {
        loop {
            if self.inner.is_emptying.load(std::sync::atomic::Ordering::Relaxed) {
                tokio::time::sleep(Duration::from_millis(100)).await;
                continue;
            }
            else {
                let mut guard = self.inner.receiver.lock().await;
                // let start = Instant::now();
                let channel = (*guard).recv().await;

                if let Some(ch) = channel {
                    let pooled_channel = PooledChannel {
                        inner: ch,
                        sender: self.inner.sender.clone(),
                    };
                    drop(guard);
                    return Ok(pooled_channel);
                }
                else {
                    drop(guard);
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    continue;
                }
            }
        }
    }
}

//when a channel leaves business logic block it is returned to channel pool
impl Drop for PooledChannel {
    fn drop(&mut self) {
        let sender = self.sender.clone();
        let channel = self.inner.clone();
        // return channel asynchronously
        tokio::spawn(async move {
            //no need to check the result because mpsc channel will never be closed since it
            let _ = sender.send(channel).await;
        });
    }
}