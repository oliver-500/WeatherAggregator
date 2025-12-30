use std::net::TcpListener;

use weather_aggregator_cache_service_app::org::unibl::etf::configuration::get_configuration;
use weather_aggregator_cache_service_app::org::unibl::etf::external_dependency_systems::redis_store::create_redis_pool;
use weather_aggregator_cache_service_app::org::unibl::etf::startup::run;

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

    let redis_connection_uri = configuration
        .redis_store
        .get_redis_config().expect("Failed to get redis connection URI");

    let redis_pool = create_redis_pool(redis_connection_uri);

    let res = run(
        listener,
        configuration.redis_store,
        redis_pool
    )?.await;

    res
}
