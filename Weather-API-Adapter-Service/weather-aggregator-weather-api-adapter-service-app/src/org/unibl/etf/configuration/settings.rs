use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub provider: ProviderSettings,
}

#[derive(Deserialize, Debug)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize, Debug)]
pub struct ProviderSettings {
    pub name: String,
    pub base_api_url: String,
    pub current_weather_endpoint: String,
    pub api_key: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub requests_per_minute: u16,
}

