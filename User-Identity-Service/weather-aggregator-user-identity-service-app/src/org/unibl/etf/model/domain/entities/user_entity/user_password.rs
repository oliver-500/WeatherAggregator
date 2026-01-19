use crate::org::unibl::etf::model::domain::errors::user_error::UserError;
use crate::org::unibl::etf::model::domain::errors::user_error::UserError::{
    PasswordDoesNotIncludeOneDigit, PasswordDoesNotIncludeOneLowercaseLetter,
    PasswordDoesNotIncludeOneUppercaseLetter, PasswordIncludesWhitespaces, PasswordTooLong,
    PasswordTooShort,
};
use secrecy::{ExposeSecret, SecretString};

#[derive(Debug, Clone)]
pub struct UserPassword(SecretString);

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
}

impl AsRef<str> for UserPassword {
    fn as_ref(&self) -> &str {
        &self.0.expose_secret()
    }
}
