
use std::net::TcpListener;
use rustls::crypto::CryptoProvider;
use weather_aggregator_aggregator_service_app::org::unibl::etf::configuration::get_configuration;
use weather_aggregator_aggregator_service_app::org::unibl::etf::startup::run;
use weather_aggregator_aggregator_service_app::org::unibl::etf::telemetry::{get_subscriber, init_subscriber};

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

    let http_client_config =
        configuration
            .application
            .get_http_client_tls_config().expect("Failed to read required web server TLS config.");

    let subscriber = get_subscriber("Aggregator Service".into(), "info".into(), std::io::stdout, configuration.tracing_agent.clone());
    init_subscriber(subscriber);

    let address = format!(
        "{}:{}",
        configuration.application.host,
        configuration.application.port
    );

    let listener = TcpListener::bind(address)
        .expect("Failed to bind to specified address.");

    let res = run(
        listener,
        configuration,
        http_server_config,
        http_client_config
    )?.await;

    res

}
