use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UserPreferencesRequest {
    pub user_id: Uuid,
}