use serde::Deserialize;
use crate::org::unibl::etf::jwt::jwkey::JwKey;

#[derive(Debug, Deserialize)]
pub struct Jwks {
    pub keys: Vec<JwKey>,
}