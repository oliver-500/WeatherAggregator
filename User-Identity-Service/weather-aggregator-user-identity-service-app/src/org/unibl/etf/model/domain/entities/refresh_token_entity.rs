use uuid::Uuid;
use crate::org::unibl::etf::model::domain::entities::user_entity::refresh_token::RefreshToken;

#[derive(Debug, Clone)]
pub struct RefreshTokenEntity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub is_revoked: bool,
    pub hashed_value: RefreshToken
}