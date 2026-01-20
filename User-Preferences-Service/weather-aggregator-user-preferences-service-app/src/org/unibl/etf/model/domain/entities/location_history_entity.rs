use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug)]
pub struct LocationHistoryEntity {
    pub lat: f64,
    pub lon: f64,
    pub location_name: String,
    pub id: Uuid,
    pub user_id: Uuid,
    pub searched_at: DateTime<Utc>,
}