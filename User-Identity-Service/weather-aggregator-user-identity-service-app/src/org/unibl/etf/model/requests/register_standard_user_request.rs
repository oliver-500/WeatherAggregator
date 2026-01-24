use secrecy::SecretBox;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RegisterStandardUserRequest {
    pub password: SecretBox<String>,
    pub email: String,
    pub use_previously_saved_data: bool
}