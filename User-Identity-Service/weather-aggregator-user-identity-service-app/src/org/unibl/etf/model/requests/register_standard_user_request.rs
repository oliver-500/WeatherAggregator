use secrecy::SecretBox;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RegisterStandardUserRequest {
    pub password: SecretBox<String>,
    pub email: String,
    pub use_previously_saved_data: bool
}