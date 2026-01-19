use secrecy::ExposeSecret;
use uuid::Uuid;
use crate::org::unibl::etf::model::domain::entities::user_entity::user_email::UserEmail;
use crate::org::unibl::etf::model::domain::entities::user_entity::user_password::UserPassword;
use crate::org::unibl::etf::model::domain::errors::user_error::UserError;
use crate::org::unibl::etf::model::requests::register_standard_user_request::RegisterStandardUserRequest;

pub mod user_email;
pub mod user_password;

#[derive(Debug, Clone)]
pub struct UserEntity {
    pub id: Uuid,
    pub password: UserPassword,
    pub email: UserEmail,
}


impl TryFrom<&RegisterStandardUserRequest> for UserEntity {
    type Error = UserError;
    fn try_from(req: &RegisterStandardUserRequest) -> Result<Self, Self::Error> {
        let user_password = UserPassword::parse(req.password.expose_secret().to_string())?;
        let user_email = UserEmail::parse(req.email.clone())?;

        Ok(UserEntity {
            id: Uuid::new_v4(),
            email: user_email,
            password: user_password,
        })
    }
}