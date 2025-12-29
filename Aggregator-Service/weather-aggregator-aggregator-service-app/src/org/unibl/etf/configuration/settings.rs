use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub providers: Vec<ProviderSettings>
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
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}
