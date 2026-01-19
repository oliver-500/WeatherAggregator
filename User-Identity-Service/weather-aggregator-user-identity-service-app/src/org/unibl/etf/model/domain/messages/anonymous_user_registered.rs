use serde::Serialize;
use uuid::Uuid;
use crate::org::unibl::etf::model::domain::entities::user_entity::UserEntity;

#[derive(Serialize, Debug)]
pub struct AnonymousUserRegistered {
    pub email: String,
    pub id: Uuid,
}

impl From<&UserEntity> for AnonymousUserRegistered {
    fn from(entity: &UserEntity) -> Self {
        Self {
            id: entity.id,
            email: entity.email.as_ref().to_string(),
        }
    }
}