use crate::org::unibl::etf::model::domain::entities::location_history_entity::LocationHistoryEntity;
use crate::org::unibl::etf::model::domain::entities::user_preferences_entity::UserPreferencesEntity;

pub struct UserPreferencesWithHistory {
    pub preferences: UserPreferencesEntity,
    pub history: Vec<LocationHistoryEntity>,
}