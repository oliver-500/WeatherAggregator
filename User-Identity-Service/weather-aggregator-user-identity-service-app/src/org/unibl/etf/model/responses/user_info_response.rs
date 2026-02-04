
use serde::Serialize;
use uuid::Uuid;
use crate::org::unibl::etf::model::domain::entities::user_entity::user_email::UserEmail;
use crate::org::unibl::etf::model::user_type::UserType;

#[derive(Serialize)]
pub struct UserInfoResponse {
    pub(crate) email: Option<UserEmail>,
    pub(crate) user_type: UserType,
    pub user_id: Uuid
}