use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct AddHistoryItemRequest {
    pub user_id: Uuid,
    #[validate(range(min = -90.0, max = 90.0))]
    pub lat: f64,

    #[validate(range(min = -180.0, max = 180.0))]
    pub lon: f64,
    pub location_name: String,
}