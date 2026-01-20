use serde::Serialize;
use uuid::Uuid;
use crate::org::unibl::etf::model::domain::entities::user_entity::UserEntity;
use crate::org::unibl::etf::model::user_type::UserType;

#[derive(Serialize, Debug)]
pub struct AnonymousUserRegistered {
    pub user_type: UserType,
    pub id: Uuid,
}

impl From<&UserEntity> for AnonymousUserRegistered {
    fn from(entity: &UserEntity) -> Self {
        Self {
            id: entity.id,
            user_type: UserType::from(entity.user_type.clone()),
        }
    }
}