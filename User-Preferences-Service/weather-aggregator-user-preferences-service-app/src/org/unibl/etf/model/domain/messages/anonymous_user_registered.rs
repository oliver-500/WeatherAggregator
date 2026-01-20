use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::org::unibl::etf::model::user_type::UserType;

#[derive(Serialize, Debug, Deserialize)]
pub struct AnonymousUserRegistered {
    pub user_type: UserType,
    pub id: Uuid,
}
