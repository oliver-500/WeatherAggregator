use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use crate::org::unibl::etf::configuration::settings::JwtSettings;
use crate::org::unibl::etf::jwt::claims::Claims;
use crate::org::unibl::etf::jwt::token_type::TokenType;
use crate::org::unibl::etf::model::user_type::UserType;

#[derive(Debug)]
pub struct JwtService {
    pub private_key: EncodingKey,
    pub public_key: DecodingKey,
    pub jwt_settings: JwtSettings,
}


impl JwtService {
    // fn new() -> Self {
    //     Self {
    //         private_key: Vec::new(),
    //         public_key: Vec::new(),
    //     }
    // }
    pub fn new(private_key: EncodingKey, public_key: DecodingKey, jwt_settings: JwtSettings) -> Self {
        Self {
            private_key,
            public_key,
            jwt_settings
        }
    }

    #[tracing::instrument(name = "Jwt service - generate token function", skip(

    ))]
    pub fn generate_token(&self, user_id: &str, user_type: UserType, token_type: TokenType) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();

        let exp = match token_type {
            TokenType::ACCESS => (now + Duration::minutes(1)).timestamp(),
            TokenType::REFRESH => (now + Duration::days(30)).timestamp(),
        };

        let claims = Claims {
            sub: user_id.to_owned(),
            user_type,
            typ: token_type,
            iat: now.timestamp() as usize,
            exp: exp as usize, // Valid for 10mins
            iss: self.jwt_settings.issuer_name.clone(),
        };

        let mut header = Header::new(Algorithm::EdDSA);
        header.kid = Some(self.jwt_settings.kid.clone());

        encode(&header, &claims, &self.private_key)
    }

    #[tracing::instrument(name = "Jwt service - validate token function", skip(

    ))]
    pub fn validate_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        // 1. Load the public key
        // 2. Define which algorithms are allowed (prevents "None" algorithm attacks)
        let mut validation = Validation::new(Algorithm::EdDSA);

        // Optional: Add extra checks (e.g., must have a specific issuer)
        validation.set_issuer(&[self.jwt_settings.issuer_name.as_str()]);

        // 3. Decode and Validate
        let token_data = decode::<Claims>(
            token,
            &self.public_key,
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
                &self.public_key,
            &validation
        );

        Ok(expired_data?.claims)
    }



}


// impl Default for JwtService {
//     fn default() -> Self {
//         Self::new()
//     }
// }
