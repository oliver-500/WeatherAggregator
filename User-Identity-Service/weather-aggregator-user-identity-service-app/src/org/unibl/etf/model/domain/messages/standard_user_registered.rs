use serde::Serialize;
use uuid::Uuid;
use crate::org::unibl::etf::model::domain::entities::user_entity::UserEntity;
use crate::org::unibl::etf::model::user_type::UserType;

#[derive(Serialize, Debug)]
pub struct StandardUserRegistered {
    pub email: String,
    pub old_id: Option<Uuid>,
    pub new_id: Uuid,
    pub user_type: UserType,
}

impl From<&UserEntity> for StandardUserRegistered {
    fn from(entity: &UserEntity) -> Self {
        Self {
            old_id: None,
            email: entity.email.as_ref().to_string(),
            new_id: entity.id.clone(),
            user_type: entity.user_type.clone(),
        }
    }
}