use actix_web::{web, HttpResponse, Responder};
use crate::org::unibl::etf::controllers::errors::{GenericServiceError, GenericServiceErrorDetails};
use crate::org::unibl::etf::model::requests::add_history_item_request::AddHistoryItemRequest;
use crate::org::unibl::etf::model::requests::update_user_preferences_request::UpdateUserPreferencesRequest;
use crate::org::unibl::etf::model::requests::user_preferences_request::UserPreferencesRequest;
use crate::org::unibl::etf::services::user_preferences_service::UserPreferencesService;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/user/preferences")
            .route(web::post().to(get_user_preferences))
            .route(web::put().to(update_user_preferences)))
        .service(web::resource("/user/history")
            .route(web::post().to(add_history_item)));

}


#[tracing::instrument(name = "User preferences controller - get user preferences function",
    skip(user_preferences_service))]
async fn get_user_preferences(
    user_preferences_service: web::Data<UserPreferencesService>,
    request_body: web::Json<UserPreferencesRequest>,
) -> Result<impl Responder, GenericServiceError> {

    match user_preferences_service.get_user_preferences(request_body.into_inner()).await {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(err) => {
            let err = GenericServiceError {
                error: GenericServiceErrorDetails::new_user_preferences_service_error(err)
            };
            Err(err)
        }
    }

}

#[tracing::instrument(name = "User preferences controller - update user preferences function",
    skip(user_preferences_service))]
async fn update_user_preferences(
    user_preferences_service: web::Data<UserPreferencesService>,
    request_body: web::Json<UpdateUserPreferencesRequest>,
) -> Result<impl Responder, GenericServiceError> {

    match user_preferences_service.update_user_preferences(request_body.into_inner()).await {
        Ok(_res) => Ok(HttpResponse::Ok().finish()),
        Err(err) => {
            let err = GenericServiceError {
                error: GenericServiceErrorDetails::new_user_preferences_service_error(err)
            };
            Err(err)
        }
    }

}

#[tracing::instrument(name = "User preferences controller - add history item function",
    skip(user_preferences_service))]
async fn add_history_item(
    user_preferences_service: web::Data<UserPreferencesService>,
    request_body: actix_web_validator::Json<AddHistoryItemRequest>,
) -> Result<impl Responder, GenericServiceError> {

    match user_preferences_service.add_history_item(request_body.into_inner()).await {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(err) => {
            let err = GenericServiceError {
                error: GenericServiceErrorDetails::new_user_preferences_service_error(err)
            };
            Err(err)
        }
    }

}