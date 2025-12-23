use std::collections::HashMap;
use std::net::TcpListener;
use wa_eaa_app::org::unibl::etf::configuration::get_configuration;
use wa_eaa_app::org::unibl::etf::configuration::settings::ProviderSettings;
use wa_eaa_app::org::unibl::etf::startup::run;

//entry point of the app, will be automatically and infinitely awaited by the macro
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //load configuration and fail fast due to criticality if not successful
    let configuration = get_configuration("resources/configuration")
        .expect("Failed to read configuration.");

    let providers: HashMap<String, ProviderSettings> = configuration
        .providers
        .into_iter()
        .map(|provider| (provider.name.clone(), provider))
        .collect();

    let address = format!(
        "{}:{}",
        configuration.application.host,
        configuration.application.port
    );
    let listener = TcpListener::bind(address)
        .expect("Failed to bind to specified address.");

    let res = run(
        listener,
        providers
    )?.await;

    res
}
