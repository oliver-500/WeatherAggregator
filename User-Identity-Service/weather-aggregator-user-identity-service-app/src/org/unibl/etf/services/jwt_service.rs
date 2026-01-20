use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use crate::org::unibl::etf::jwt::claims::Claims;
use crate::org::unibl::etf::model::user_type::UserType;

#[derive(Debug)]
pub struct JwtService {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}


impl JwtService {
    fn new() -> Self {
        Self {
            private_key: Vec::new(),
            public_key: Vec::new(),
        }
    }
    pub fn new_with_private_key(private_key: Vec<u8>, public_key: Vec<u8> ) -> Self {
        Self {
            private_key,
            public_key
        }
    }

    #[tracing::instrument(name = "Jwt service - generate token function", skip(

    ))]
    pub fn generate_token(&self, user_id: &str, user_type: UserType) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let my_claims = Claims {
            sub: user_id.to_owned(),
            user_type: user_type,
            iat: now.timestamp() as usize,
            exp: (now + Duration::minutes(10)).timestamp() as usize, // Valid for 10mins
            iss: "weather-aggregator-user-identity-service-app".to_owned(),
        };

        let header = Header::new(Algorithm::EdDSA);
        let key = EncodingKey::from_ed_pem(&self.private_key)?;

        encode(&header, &my_claims, &key)
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
            &DecodingKey::from_ed_pem(&self.public_key)?,
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
            &DecodingKey::from_ed_pem(&self.public_key)?,
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
