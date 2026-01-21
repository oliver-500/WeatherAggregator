pub mod unit_system_type;

use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::org::unibl::etf::model::domain::entities::user_preferences_entity::unit_system_type::UnitSystemType;
use crate::org::unibl::etf::model::requests::update_user_preferences_request::UpdateUserPreferencesRequest;
use crate::org::unibl::etf::model::user_type::UserType;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct UserPreferencesEntity {
    pub user_id: Uuid,
    pub user_type: Option<UserType>,
    pub unit_system: UnitSystemType,
    pub favorite_location_name: Option<String>,
    pub favorite_lat: Option<f64>,
    pub favorite_lon: Option<f64>,

    pub updated_at: DateTime<Utc>,
}


impl From<UpdateUserPreferencesRequest> for UserPreferencesEntity {

    fn from(value: UpdateUserPreferencesRequest) -> Self {

        UserPreferencesEntity {
            user_id: value.user_id,
            user_type: None,
            unit_system: value.unit_system,
            favorite_location_name: value.favorite_location_name,
            favorite_lat: value.favorite_lat,
            favorite_lon: value.favorite_lon,
            updated_at: Default::default(),
        }
    }
}