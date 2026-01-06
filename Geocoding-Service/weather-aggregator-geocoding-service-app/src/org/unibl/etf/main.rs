use std::net::TcpListener;
use weather_aggregator_geocoding_service_app::org::unibl::etf::configuration::get_configuration;
use weather_aggregator_geocoding_service_app::org::unibl::etf::external_dependency_systems::redis_store::create_redis_pool;
use weather_aggregator_geocoding_service_app::org::unibl::etf::startup::run;
use weather_aggregator_geocoding_service_app::org::unibl::etf::telemetry::{get_subscriber, init_subscriber};
use rustls::crypto::CryptoProvider;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration("resources/configuration")
        .expect("Failed to read configuration.");

    //install crypto provider and fail fast due to criticality if not successful
    CryptoProvider::install_default(rustls::crypto::ring::default_provider())
        .expect("Failed to install CryptoProvider.");

    let http_server_config =
        configuration
            .application
            .get_http_server_tls_config().expect("Failed to read required web server TLS config.");

    let subscriber = get_subscriber("Geocoding Service".into(), "info".into(), std::io::stdout, configuration.tracing_agent.clone());
    init_subscriber(subscriber);

    let address = format!(
        "{}:{}",
        configuration.application.host,
        configuration.application.port
    );
    let listener = TcpListener::bind(address)
        .expect("Failed to bind to specified address.");

    let redis_connection_uri = configuration
        .redis_store
        .get_redis_config().expect("Failed to get redis connection URI");

    let redis_pool = create_redis_pool(redis_connection_uri);

    let res = run(
        listener,
        configuration.geocoding_api,
        redis_pool,
        http_server_config
    )?.await;

    res
}
