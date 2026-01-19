use crate::org::unibl::etf::model::domain::errors::user_error::UserError;

#[derive(Debug, Clone)]
pub struct UserEmail(pub String);

impl UserEmail {
    pub fn parse(s: String) -> Result<UserEmail, UserError> {
        if validator::ValidateEmail::validate_email(&s) {
            Ok(UserEmail(s))
        } else {
            Err(UserError::EmailInIncorrectFormat)
        }
    }
}

impl AsRef<str> for UserEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
