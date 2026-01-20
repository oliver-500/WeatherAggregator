use secrecy::SecretBox;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LoginStandardUserRequest {
    pub email: String,
    pub password: SecretBox<String>,
}