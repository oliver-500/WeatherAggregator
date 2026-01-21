use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct AddHistoryItemResponse {
    pub location_id: Uuid,
}