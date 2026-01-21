use serde::Serialize;
use crate::org::unibl::etf::model::domain::entities::location_history_entity::LocationHistoryEntity;
use crate::org::unibl::etf::model::domain::entities::user_preferences_entity::UserPreferencesEntity;

#[derive(Serialize, Debug)]
pub struct UserPreferencesWithHistory {
    pub preferences: UserPreferencesEntity,
    pub history: Vec<LocationHistoryEntity>,
}