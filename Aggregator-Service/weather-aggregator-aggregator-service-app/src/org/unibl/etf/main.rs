
use std::net::TcpListener;
use weather_aggregator_aggregator_service_app::org::unibl::etf::configuration::get_configuration;
use weather_aggregator_aggregator_service_app::org::unibl::etf::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration("resources/configuration")
        .expect("Failed to read configuration.");

    let address = format!(
        "{}:{}",
        configuration.application.host,
        configuration.application.port
    );


    let listener = TcpListener::bind(address)
        .expect("Failed to bind to specified address.");

    let res = run(
        listener,
        configuration.providers
    )?.await;

    res

}
