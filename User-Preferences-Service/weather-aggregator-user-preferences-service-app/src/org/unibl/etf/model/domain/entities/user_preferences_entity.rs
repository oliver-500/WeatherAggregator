pub mod unit_system_type;

use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::org::unibl::etf::model::domain::entities::user_preferences_entity::unit_system_type::UnitSystemType;
use crate::org::unibl::etf::model::user_type::UserType;

#[derive(Debug, sqlx::FromRow)]
pub struct UserPreferencesEntity {
    pub user_id: Uuid,
    pub user_type: UserType,
    pub unit_system: UnitSystemType,
    pub favorite_location_name: Option<String>,
    pub favorite_lat: Option<f64>,
    pub favorite_lon: Option<f64>,

    pub updated_at: DateTime<Utc>,
}