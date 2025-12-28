use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub geocoding_api: GeocodingAPISettings,
}

#[derive(Deserialize, Debug)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(Deserialize, Debug)]
pub struct GeocodingAPISettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub requests_per_minute: u16,
    pub api_key: String,
    pub endpoint: String,
}

