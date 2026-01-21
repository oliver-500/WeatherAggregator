use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use crate::org::unibl::etf::model::requests::add_history_item_request::AddHistoryItemRequest;

#[derive(Debug, Serialize)]
pub struct LocationHistoryEntity {
    pub lat: f64,
    pub lon: f64,
    pub location_name: String,
    pub id: Uuid,
    pub user_id: Uuid,
    pub searched_at: DateTime<Utc>,
}


impl From<AddHistoryItemRequest> for LocationHistoryEntity {

    fn from(r: AddHistoryItemRequest) -> Self {
        LocationHistoryEntity {
            lat: r.lat,
            lon: r.lon,
            location_name: r.location_name,
            id: Default::default(),
            user_id: r.user_id,
            searched_at: Default::default(),
        }
    }

}
