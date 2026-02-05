

use secrecy::{SecretString};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct RefreshToken(pub SecretString);


impl RefreshToken {
    pub fn hash_refresh_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    // When verifying, use a constant-time comparison!
    pub fn verify_token(provided_token: &str, stored_hash: &str) -> bool {
        let hashed_provided = RefreshToken::hash_refresh_token(provided_token);

        println!("hashed refresh token: {} vs  {}", hashed_provided, stored_hash);
        // Use a crate like `subtle` to prevent timing attacks
        subtle::ConstantTimeEq::ct_eq(hashed_provided.as_bytes(), stored_hash.as_bytes()).into()
    }
}

