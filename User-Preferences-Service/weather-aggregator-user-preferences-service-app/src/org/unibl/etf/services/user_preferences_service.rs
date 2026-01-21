
use crate::org::unibl::etf::model::domain::entities::location_history_entity::LocationHistoryEntity;
use crate::org::unibl::etf::model::domain::entities::user_preferences_entity::UserPreferencesEntity;
use crate::org::unibl::etf::model::domain::entities::user_preferences_with_history::UserPreferencesWithHistory;
use crate::org::unibl::etf::model::errors::user_identity_service_error::UserPreferencesServiceError;
use crate::org::unibl::etf::model::requests::add_history_item_request::AddHistoryItemRequest;
use crate::org::unibl::etf::model::requests::update_user_preferences_request::UpdateUserPreferencesRequest;
use crate::org::unibl::etf::model::requests::user_preferences_request::UserPreferencesRequest;
use crate::org::unibl::etf::model::responses::add_history_item_response::AddHistoryItemResponse;
use crate::org::unibl::etf::model::user_type::UserType;
use crate::org::unibl::etf::repositories::user_preferences_repository::UserPreferencesRepository;

#[derive(Debug)]
pub struct UserPreferencesService {
    pub user_preferences_repository: UserPreferencesRepository,
}

impl UserPreferencesService {

    #[tracing::instrument(name = "User Preferences service - get user preferences function", skip(
        self
    ))]
    pub async fn get_user_preferences(
        &self,
        res: UserPreferencesRequest
    ) -> Result<UserPreferencesWithHistory, UserPreferencesServiceError> {
        match self.user_preferences_repository
            .find_by_id_with_history(res.user_id)
            .await {
            Ok(res) => {
                Ok(res)
            },
            Err(error) => {
                tracing::error!("Error while fetching user preferences: {}", error);
                match error {
                    sqlx::Error::RowNotFound => {
                        return Err(UserPreferencesServiceError::UserError(Some("User with that id does not exist".to_string())))
                    }
                    _ => return Err(UserPreferencesServiceError::DatabaseError(Some(format!("Could not fetch user preferences from database with error: {}", error.to_string()))))
                }
            }
        }
    }

    #[tracing::instrument(name = "User Preferences service - update user preferences function", skip(
        self
    ))]
    pub async fn update_user_preferences(
        &self,
        res: UpdateUserPreferencesRequest
    ) -> Result<(), UserPreferencesServiceError> {
        let data : UserPreferencesEntity = res.into();

        match self.user_preferences_repository
            .save(&data)
            .await {
            Ok(res) => {
                Ok(res)
            },
            Err(error) => {
                tracing::error!("Error while fetching user preferences: {}", error);
                match error {
                    sqlx::Error::RowNotFound => {
                        return Err(UserPreferencesServiceError::UserError(Some("User with that id does not exist".to_string())))
                    }
                    _ => return Err(UserPreferencesServiceError::DatabaseError(Some(format!("Could not update user preferences in database with error: {}", error.to_string()))))
                }
            }
        }
    }


    #[tracing::instrument(name = "User Preferences service - add history item function", skip(
        self
    ))]
    pub async fn add_history_item(
        &self,
        res: AddHistoryItemRequest,
    ) -> Result<AddHistoryItemResponse, UserPreferencesServiceError> {
        let user_type = match self.user_preferences_repository
            .find_by_id_with_history(res.user_id)
            .await {
            Err(e) => {
                tracing::error!("Error while fetching user info: {}", e);
                return Err(UserPreferencesServiceError::DatabaseError(Some("Error while fetching user info needed for retrieving history data".to_string())))
            },
            Ok(user_info) => user_info.preferences.user_type
        };




        let limit = match user_type {
            Some(user_type) => {
                match user_type {
                    UserType::GUEST => 5,    // Guests get 5 items
                    UserType::STANDARD => 20, //
                }
            },
            None => {
                tracing::error!("Could not determine user type.");
                return Err(UserPreferencesServiceError::ServerError(None))
            }
        };

        let data : LocationHistoryEntity = res.into();

        match self.user_preferences_repository
            .add_history_item(data, limit)
            .await {
            Ok(id) => {
                Ok(AddHistoryItemResponse {
                    location_id: id
                })
            },
            Err(e) => {
                tracing::error!("Error while saving history item data {}", e);
                return Err(UserPreferencesServiceError::DatabaseError(Some(format!("Error while saving history item: {}", e.to_string()))))
            }
        }
    }
}



