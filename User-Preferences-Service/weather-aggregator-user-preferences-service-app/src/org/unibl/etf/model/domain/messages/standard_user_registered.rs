use serde::{Deserialize};
use uuid::Uuid;
use crate::org::unibl::etf::model::user_type::UserType;

#[derive(Deserialize, Debug)]
pub struct StandardUserRegistered {
    pub email: String,
    pub old_id: Option<Uuid>,
    pub new_id: Uuid,
    pub user_type: UserType,
}