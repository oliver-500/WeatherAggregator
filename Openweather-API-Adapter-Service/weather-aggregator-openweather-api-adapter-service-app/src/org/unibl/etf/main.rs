use std::net::TcpListener;
use weather_aggregator_openweather_api_adapter_service_app::org::unibl::etf::configuration::get_configuration;
use weather_aggregator_openweather_api_adapter_service_app::org::unibl::etf::startup::run;
use weather_aggregator_openweather_api_adapter_service_app::org::unibl::etf::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration("resources/configuration")
        .expect("Failed to read configuration.");

    let subscriber = get_subscriber("Openweather API Adapter Service".into(), "info".into(), std::io::stdout, configuration.tracing_agent.clone());
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
        configuration
    )?.await;

    res
}
