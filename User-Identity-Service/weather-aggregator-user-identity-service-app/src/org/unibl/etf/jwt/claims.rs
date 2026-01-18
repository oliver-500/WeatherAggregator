use serde::{Deserialize, Serialize};
use crate::org::unibl::etf::model::user_type::UserType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,      // User ID
    // pub email: String,    // Custom claim
    pub user_type: UserType, // Custom claim (e.g., "guest", "standard")
    pub exp: usize,       // Required for security
    pub iat: usize,       // Issued At
    pub iss: String,
}