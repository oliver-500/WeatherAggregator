
use serde::Deserialize;
use uuid::Uuid;
use crate::org::unibl::etf::model::domain::entities::user_preferences_entity::unit_system_type::UnitSystemType;


#[derive(Debug, Deserialize)]
pub struct UpdateUserPreferencesRequest {
    pub user_id: Uuid,
    pub unit_system: UnitSystemType,
    pub favorite_location_name: Option<String>,
    pub favorite_lat: Option<f64>,
    pub favorite_lon: Option<f64>,
}