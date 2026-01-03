use std::{fs, io};
use opentelemetry_otlp::tonic_types::transport::{Certificate, ClientTlsConfig, Identity};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_aux::prelude::deserialize_bool_from_anything;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub geocoding_api: GeocodingAPISettings,
    pub tracing_agent: TracingSettings,
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


#[derive(serde::Deserialize, Debug, Clone)]
pub struct TracingSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub require_tls: bool,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub mtls_enabled: bool,
    pub client_certificate_pem_file_path: String,
    pub ca_certificate_pem_file_path: String,
    pub client_certificate_pem_key_path: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub max_export_batch_size: u16,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub max_queue_size: u16,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub scheduled_delay_in_ms: u16,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub max_concurrent_exports: u16,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timeout_in_ms: u16,
    pub domain_name: String,
}

impl TracingSettings {
    pub fn get_tls_config(&self) -> Result<Option<ClientTlsConfig>, io::Error> {
        let mut client_tls_config = ClientTlsConfig::new();

        println!("tls: {}, mtls: {}", self.require_tls, self.mtls_enabled);

        if self.require_tls {
            let ca_pem_bytes = fs::read_to_string(&self.ca_certificate_pem_file_path)?;
            let ca_certificate = Certificate::from_pem(ca_pem_bytes);

            client_tls_config = client_tls_config.domain_name(&self.domain_name);

            client_tls_config = client_tls_config
                .ca_certificate(ca_certificate);

            if self.mtls_enabled {
                let identity = Identity::from_pem(
                    fs::read(&self.client_certificate_pem_file_path)?,
                    fs::read(&self.client_certificate_pem_key_path)?
                );

                client_tls_config = client_tls_config.identity(
                    identity
                );

            }
            return Ok(Some(client_tls_config));
        }

        return Ok(None)
    }
}