use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use lapin::{Channel, Connection};
use tokio::sync::Mutex;

use crate::org::unibl::etf::external_dependency_systems::message_broker::broker_util::TLSConfigDataSource;
use crate::org::unibl::etf::external_dependency_systems::message_broker::channel_pool::PreparingChannel;

#[derive(Clone)]
pub enum BrokerTask {
    ConnectBroker {
        connection_uri: String,
        tls_config_source: Arc<Option<TLSConfigDataSource>>,
        connection: Arc<Mutex<Option<Connection>>>,
        is_broker_up: Arc<AtomicBool>,
    },
    PublishMessageToBroker {
        payload: Vec<u8>,
        routing_key: String,
    },
    CreateChannelToBroker {
        connection: Arc<Mutex<Option<Connection>>>,
        channel_tx: tokio::sync::mpsc::Sender<Channel>,
    },
    ConfigureChannelToBroker {
        channel: Arc<Mutex<Option<PreparingChannel>>>,
    },
    AssureNeededPermissionsToBroker {
        channel: Arc<Mutex<Option<PreparingChannel>>>,
    },
}