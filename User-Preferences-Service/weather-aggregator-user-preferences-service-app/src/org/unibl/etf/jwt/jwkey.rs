use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JwKey {
    kid: String,
    pub(crate) x: String, // The public key part
}

