use std::{env, fs, io};
use std::io::BufReader;
use std::str::FromStr;
use std::sync::Arc;
use opentelemetry_otlp::tonic_types::transport::{Certificate, ClientTlsConfig, Identity};
use rustls::{RootCertStore, ServerConfig};
use rustls::server::WebPkiClientVerifier;
use secrecy::{ExposeSecret, SecretBox};
use serde_aux::prelude::deserialize_bool_from_anything;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::ConnectOptions;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use tracing::log::LevelFilter;
use crate::org::unibl::etf::external_dependency_systems::message_broker::broker_util::{TLSConfigDataSource, TlsOwnedIdentityPKCS12};

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub tracing_agent: TracingSettings,
    pub jwt: JwtSettings,
    pub broker: BrokerSettings,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretBox<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub ssl_mode: String,
    pub root_ca_certificate_pem_file_path: String,
    pub client_certificate_pem_file_path: String,
    pub client_certificate_pem_key_path: String
}

impl DatabaseSettings {
    pub fn with_db(&self) -> Result<PgConnectOptions, sqlx::error::Error> {
        let options = self.without_db()?.database(&self.database_name);
        Ok(options)
    }

    pub fn without_db(&self) -> Result<PgConnectOptions, sqlx::error::Error> {
        let ssl_mode = PgSslMode::from_str(self.ssl_mode.as_str())?;

        let options = PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
            .ssl_root_cert(&self.root_ca_certificate_pem_file_path)
            .ssl_client_cert(&self.client_certificate_pem_file_path)
            .ssl_client_key(&self.client_certificate_pem_key_path);

        let env_var_name = "DATABASE_LOG_LEVEL";

        let log_level_str = env::var(env_var_name)
            .unwrap_or_else(|_| {
                // Provide a default value if the env var is not found or is invalid
                eprintln!("{} not set, defaulting to 'info'.", env_var_name);
                "info".to_string()
            });

        let log_level = LevelFilter::from_str(&log_level_str)
            // .unwrap_or_else() is used here to panic with a descriptive message
            // if the string (e.g., "invalid_level") is not a valid log level.
            .unwrap_or_else(|_| {
                panic!(
                    "Invalid value for {}: '{}'. Must be one of: trace, debug, info, warn, error, off.",
                    env_var_name, log_level_str);
            });
        // 2. Use std::env::var() to read the variable.


        //panic if env variable is not in possible range
        Ok(options.log_statements(log_level))
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct BrokerSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub require_ssl: bool,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub require_mtls: bool,
    pub client_certificate_p12_file_path: String,
    pub ca_certificate_pem_file_path: String,
    pub client_certificate_p12_file_password: String,

    pub user_password: SecretBox<String>,
    pub username: String,
    pub vhost: String,
}


impl BrokerSettings {
    pub fn  get_broker_config(&self) -> Result<(String, Option<TLSConfigDataSource>), io::Error> {
        let mut tls_config_source = None;

        if self.require_ssl {
            let ca_cert_in_pem_format = fs::read_to_string(&self.ca_certificate_pem_file_path)?;
            let client_cert_in_der_format;
            let mut identity = None;

            if self.require_mtls {
                client_cert_in_der_format = Some(fs::read(&self.client_certificate_p12_file_path)?);

                identity = Some(TlsOwnedIdentityPKCS12 {
                    der: client_cert_in_der_format.unwrap(),
                    password: self.client_certificate_p12_file_password.clone(),
                });
            }
            tls_config_source = Some(TLSConfigDataSource {
                identity,
                cert_chain: Some(ca_cert_in_pem_format),
            });
        }

        let scheme = match self.require_ssl {
            true => {
                "amqps"
            },
            false => {
                "amqp"
            }
        };

        let connection_uri = format!("{}://{}:{}@{}:{}/{}",
                                     scheme, &self.username,
                                     &self.user_password.expose_secret(),
                                     &self.host, &self.port,
                                     urlencoding::encode(&self.vhost));

        Ok((connection_uri, tls_config_source))
    }
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

#[derive(serde::Deserialize, Debug, Clone)]
pub struct JwtSettings {
    pub private_key_file_path: String,
    pub public_key_file_path: String,
}

impl JwtSettings {
    pub fn get_jwt_private_key(&self) -> Result<Vec<u8>, io::Error> {
        fs::read(self.private_key_file_path.clone())
    }
    pub fn get_jwt_public_key(&self) -> Result<Vec<u8>, io::Error> {
        fs::read(self.public_key_file_path.clone())
    }
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

    // pub fn get_http_client_tls_config(&self) -> Result<HttpClientTlsIdentityBundle, io::Error> {
    //
    //     let der = fs::read(self.pkcs_file_path.clone())?;
    //     let password = &self.pkcs_export_password;
    //
    //     let identity = reqwest::tls::Identity::from_pkcs12_der(&der, password.expose_secret().clone().as_str());
    //     let identity = match identity {
    //         Ok(identity) => identity,
    //         Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    //     };
    //
    //     let ca_cert = fs::read(self.ca_cert_file_path.clone())?;
    //     let ca_certificate = reqwest::tls::Certificate::from_pem(&ca_cert);
    //     let ca_certificate = match ca_certificate {
    //         Ok(ca_certificate) => ca_certificate,
    //         Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    //     };
    //
    //     Ok(HttpClientTlsIdentityBundle {
    //         identity,
    //         ca_certificate,
    //     })
    // }
}

// pub struct HttpClientTlsIdentityBundle {
//     pub identity: reqwest::Identity,
//     pub ca_certificate: reqwest::Certificate,
// }