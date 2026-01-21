
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use crate::org::unibl::etf::jwt::claims::Claims;


#[derive(Debug, Clone)]
pub struct JwtService {

    pub signer_public_key: Vec<u8>,
}


impl JwtService {
    fn new() -> Self {
        Self {
            signer_public_key: Vec::new(),
        }
    }
    pub fn new_with_signer_private_key(signer_public_key: Vec<u8> ) -> Self {
        Self {
            signer_public_key
        }
    }


    #[tracing::instrument(name = "Jwt service - validate token function", skip(

    ))]
    pub fn validate_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        // 1. Load the public key
        // 2. Define which algorithms are allowed (prevents "None" algorithm attacks)
        let mut validation = Validation::new(Algorithm::EdDSA);

        // Optional: Add extra checks (e.g., must have a specific issuer)
        validation.set_issuer(&["weather-aggregator-user-identity-service-app".to_owned()]);

        // 3. Decode and Validate
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_ed_pem(&self.signer_public_key)?,
            &validation
        )?;

        // If successful, return the claims
        Ok(token_data.claims)
    }

    pub fn get_claims_from_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::new(Algorithm::EdDSA);
        validation.validate_exp = false; // <--- The key setting

        let expired_data = decode::<Claims>(
            token,
            &DecodingKey::from_ed_pem(&self.signer_public_key)?,
            &validation
        );

        Ok(expired_data?.claims)
    }
}


impl Default for JwtService {
    fn default() -> Self {
        Self::new()
    }
}
