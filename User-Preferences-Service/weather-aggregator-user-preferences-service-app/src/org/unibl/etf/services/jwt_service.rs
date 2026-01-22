
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use crate::org::unibl::etf::configuration::settings::JwtSettings;
use crate::org::unibl::etf::jwt::claims::Claims;


#[derive(Debug, Clone)]
pub struct JwtService {
    pub signer_public_key: DecodingKey,

    jwt_settings: JwtSettings
}


impl JwtService {
    // fn new() -> Self {
    //     Self {
    //         signer_public_key: Vec::new(),
    //     }
    // }
    pub fn new(
        signer_public_key: DecodingKey,

        jwt_settings: JwtSettings,
    ) -> Self {
        Self {
            signer_public_key,

            jwt_settings
        }
    }


    #[tracing::instrument(name = "Jwt service - validate token function", skip(

    ))]
    pub fn validate_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let header = jsonwebtoken::decode_header(token)?;
        println!("JWT Header: {:#?}", header);
        // Optional: Verify this token belongs to the 'v1' key (or whatever your kid is)
        if let Some(kid) = header.kid {
            if kid != "v1" {
                // Return an error if the kid doesn't match your loaded key
                return Err(jsonwebtoken::errors::ErrorKind::InvalidKeyFormat.into());
            }
        }


        // 1. Load the public key
        // 2. Define which algorithms are allowed (prevents "None" algorithm attacks)
        let mut validation = Validation::new(Algorithm::EdDSA);

        // Optional: Add extra checks (e.g., must have a specific issuer)
        validation.set_issuer(&[self.jwt_settings.issuer.clone()]);

        // 3. Decode and Validate
        let token_data = decode::<Claims>(
            token,
            &self.signer_public_key,
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
            &self.signer_public_key,
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
