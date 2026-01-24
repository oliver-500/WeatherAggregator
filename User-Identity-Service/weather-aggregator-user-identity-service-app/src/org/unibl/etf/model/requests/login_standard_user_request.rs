use secrecy::SecretBox;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct LoginStandardUserRequest {
    pub email: String,
    pub password: SecretBox<String>,
}