use std::{fs, io};
use std::io::BufReader;
use std::sync::Arc;
use opentelemetry_otlp::tonic_types::transport::{Certificate, ClientTlsConfig, Identity};
use rustls::{RootCertStore, ServerConfig};
use rustls::server::WebPkiClientVerifier;
use secrecy::{ExposeSecret, SecretBox};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use serde_aux::prelude::deserialize_bool_from_anything;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub geocoding_api: GeocodingAPISettings,
    pub tracing_agent: TracingSettings,
    pub redis_store: RedisStoreSettings,
}

#[derive(Deserialize, Debug)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub tls_enabled: bool,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub require_mtls: bool,
    pub cert_file_path: String,
    pub private_key_file_path: String,
    pub pkcs_file_path: String,
    pub pkcs_export_password: SecretBox<String>,
    pub ca_cert_file_path: String,
}

#[derive(Deserialize, Debug)]
pub struct GeocodingAPISettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub requests_per_30_mins: u64,
    pub api_key: SecretBox<String>,
    pub endpoint: String,
    pub provider: String,
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
    pub scheme: String,
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



impl RedisStoreSettings {
    pub fn get_redis_config(&self) -> Result<String, io::Error> {
        let connection_uri = format!("{}://{}:{}@{}:{}",
                                     self.scheme, &self.username,
                                     &self.user_password.expose_secret(),
                                     &self.host, &self.port);
        return Ok(connection_uri)
    }
}

impl ApplicationSettings {
    pub fn get_http_server_tls_config(&self) -> Result<Option<ServerConfig>, io::Error> {

        if !self.tls_enabled {
            return Ok(None);
        }


        let server_cert_file =
            &mut BufReader::new(fs::File::open(self.cert_file_path.clone()).unwrap());
        let server_key_file =
            &mut BufReader::new(fs::File::open(self.private_key_file_path.clone()).unwrap());

        let cert_chain = rustls_pemfile::certs(server_cert_file)
            .collect::<Result<Vec<_>, _>>().unwrap();
        let keys = rustls_pemfile::pkcs8_private_keys(server_key_file)
            .next().unwrap().unwrap();

        if !self.require_mtls {
            return Ok(Some(ServerConfig::builder()
                .with_no_client_auth()
                .with_single_cert(cert_chain, keys.into())
                .unwrap()
            ));
        }

        let mut roots = RootCertStore::empty();
        let cert_file = &mut BufReader::new(fs::File::open(self.ca_cert_file_path.clone())?);

        for cert in rustls_pemfile::certs(cert_file) {
            roots.add(cert.unwrap()).unwrap();
        }

        let client_verifier = WebPkiClientVerifier::builder(Arc::new(roots))
            .build()
            .unwrap();

        let config = ServerConfig::builder()
            .with_client_cert_verifier(client_verifier) // Enforce client auth
            .with_single_cert(cert_chain, keys.into())
            .unwrap();

        Ok(Some(config))
    }
}
