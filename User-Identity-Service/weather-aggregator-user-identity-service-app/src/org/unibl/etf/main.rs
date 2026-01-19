
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use rustls::crypto::CryptoProvider;
use tokio::sync::mpsc;
use weather_aggregator_user_identity_service_app::org::unibl::etf::configuration::get_configuration;
use weather_aggregator_user_identity_service_app::org::unibl::etf::database::{connect_to_db, spawn_db_monitor};
use weather_aggregator_user_identity_service_app::org::unibl::etf::external_dependency_systems::message_broker::broker_task::BrokerTask;
use weather_aggregator_user_identity_service_app::org::unibl::etf::external_dependency_systems::message_broker::broker_task_handler::broker_task_handler;
use weather_aggregator_user_identity_service_app::org::unibl::etf::external_dependency_systems::message_broker::channel_pool::ChannelPool;
use weather_aggregator_user_identity_service_app::org::unibl::etf::startup::run;
use weather_aggregator_user_identity_service_app::org::unibl::etf::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration("resources/configuration")
        .expect("Failed to read configuration.");

    //install crypto provider and fail fast due to criticality if not successful
    CryptoProvider::install_default(rustls::crypto::ring::default_provider())
        .expect("Failed to install CryptoProvider.");

    let jwt_private_key = configuration
        .jwt
        .get_jwt_private_key()
        .expect("Unable to read required private key for jwt.");

    let jwt_public_key = configuration
        .jwt
        .get_jwt_public_key()
        .expect("Unable to read required public key for jwt.");

    let http_server_config = configuration
        .application
        .get_http_server_tls_config()
        .expect("Failed to read required web server TLS config.");

    let database_settings = configuration
        .database
        .with_db()
        .expect("Failed to configure Postgres database connection.");

    let (broker_connection_uri, tls_config_source) = configuration
        .broker
        .get_broker_config()
        .expect("Failed to configure RabbitMQ message broker connection.");

    let subscriber = get_subscriber("User Identity Service".into(), "info".into(), std::io::stdout, configuration.tracing_agent.clone());
    init_subscriber(subscriber);

    let address = format!(
        "{}:{}",
        configuration.application.host,
        configuration.application.port
    );

    let is_db_up = Arc::new(AtomicBool::new(false));
    let db_connection_pool = match connect_to_db(database_settings)
        .await {
        Ok(connection_pool) => connection_pool,
        Err(e) => {
            panic!("Failed to create database connection pool: {}", e.to_string());
        }
    };
    spawn_db_monitor(db_connection_pool.clone(), is_db_up.clone());


    let (tx, rx) = mpsc::channel::<BrokerTask>(50);
    let tx_pointer = tx.clone();

    tokio::spawn(async move {
        broker_task_handler(rx, tx_pointer).await;
    });

    let is_broker_up = Arc::new(AtomicBool::new(false));
    let broker_channel_pool = Arc::new(
        match ChannelPool::new(20).await {
            Ok(pool) => {
                pool
            }
            Err(e) => {
                panic!("Failed to create message broker channel pool: {}", e.to_string());
            }
        }
    );

    let broker_channel_pool_pointer = broker_channel_pool.clone();
    let is_broker_up_pointer = is_broker_up.clone();
    tokio::spawn(async move {
        broker_channel_pool_pointer.configure_channel_pool(
            broker_connection_uri.clone(),
            tls_config_source.clone(),
            tx,
            is_broker_up_pointer
        ).await;
    });


    let listener = TcpListener::bind(address)
        .expect("Failed to bind to specified address.");
    //conditional bloker kad implementiram sve endpointe
    let res = run(
        listener,
        configuration,
        http_server_config,
        jwt_private_key,
        jwt_public_key,
        is_broker_up,
        is_db_up,
        db_connection_pool,
        broker_channel_pool
    )?.await;

    res

}
