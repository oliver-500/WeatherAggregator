use std::io;
use secrecy::{ExposeSecret, SecretBox};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub redis_store: RedisStoreSettings,
}

#[derive(Deserialize, Debug)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct RedisStoreSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub scheme: String,
    pub username: String,
    pub user_password: SecretBox<String>
}


impl RedisStoreSettings {
    pub fn get_redis_config(&self) -> Result<String, io::Error> {
        let connection_uri = format!("{}://{}:{}@{}:{}",
                                     self.scheme, &self.username,
                                     &self.user_password.expose_secret(),
                                     &self.host, &self.port);
        return Ok(connection_uri)
    }
}