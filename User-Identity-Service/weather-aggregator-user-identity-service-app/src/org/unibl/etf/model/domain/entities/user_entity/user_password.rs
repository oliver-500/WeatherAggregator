use argon2::Argon2;
use password_hash::rand_core::OsRng;
use password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use crate::org::unibl::etf::model::domain::errors::user_error::UserError;
use crate::org::unibl::etf::model::domain::errors::user_error::UserError::{
    PasswordDoesNotIncludeOneDigit, PasswordDoesNotIncludeOneLowercaseLetter,
    PasswordDoesNotIncludeOneUppercaseLetter, PasswordIncludesWhitespaces, PasswordTooLong,
    PasswordTooShort,
};
use secrecy::{ExposeSecret, SecretString};

#[derive(Debug, Clone)]
pub struct UserPassword(pub SecretString);

impl UserPassword {
    pub fn parse(pw: String) -> Result<UserPassword, UserError> {
        if pw.len() < 8 {
            return Err(PasswordTooShort);
        }
        if pw.len() > 20 {
            return Err(PasswordTooLong);
        }
        if !pw.chars().any(|c| c.is_ascii_uppercase()) {
            return Err(PasswordDoesNotIncludeOneUppercaseLetter);
        }
        if !pw.chars().any(|c| c.is_ascii_lowercase()) {
            return Err(PasswordDoesNotIncludeOneLowercaseLetter);
        }
        if !pw.chars().any(|c| c.is_ascii_digit()) {
            return Err(PasswordDoesNotIncludeOneDigit);
        }
        if pw.chars().any(|c| c.is_whitespace()) {
            return Err(PasswordIncludesWhitespaces);
        }
        Ok(Self(SecretString::from(pw)))
    }

    // #[allow(dead_code)]
    // fn check_password_strength(password: &str, user_inputs: &[&str]) -> Result<(), String> {
    //     let estimate = zxcvbn::zxcvbn(password, user_inputs);
    //
    //     if estimate.score().lt(&zxcvbn::Score::Three) {
    //         return Err("Password is too weak.".into());
    //     }
    //
    //     Ok(())
    //}

    pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        // The output string includes: algorithm, version, params, salt, and hash.
        // Example: $argon2id$v=19$m=4096,t=3,p=1$sH8...
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(password_hash)
    }

    pub fn verify_password(password: &str, stored_hash: &str) -> Result<bool, password_hash::Error> {
        let parsed_hash = PasswordHash::new(stored_hash)?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

impl AsRef<str> for UserPassword {
    fn as_ref(&self) -> &str {
        &self.0.expose_secret()
    }
}

