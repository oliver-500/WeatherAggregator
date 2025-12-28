use actix_web::{web, HttpResponse, Responder};
use actix_web_validator::Query;
use reqwest::Client;
use crate::org::unibl::etf::configuration::settings::{GeocodingAPISettings};
use crate::org::unibl::etf::controllers::errors::generic_service_error::{GenericServiceError, GenericServiceErrorDetails};
use crate::org::unibl::etf::model::requests::geocoding_request::GeocodingRequest;
use crate::org::unibl::etf::model::responses::geocoding_response::GeocodingResponse;
use crate::org::unibl::etf::services::geocoding_service::GeocodingService;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/geocode").route(web::get().to(get_coordinates_by_city_name)));
}

async fn get_coordinates_by_city_name(
    geocoding_service: web::Data<GeocodingService>,
    query: Query<GeocodingRequest>,
    http_client: web::Data<Client>,
    settings: web::Data<GeocodingAPISettings>
) -> Result<impl Responder, GenericServiceError> {
    Ok(geocoding_service
        .geocode_location(
            &query.location_name,
            query.limit.unwrap(),
            http_client.get_ref(),
            settings.get_ref()
        )
        .await
        .and_then(|candidates| {
            println!("{:?}", candidates);
            Ok(HttpResponse::Ok().json(
            GeocodingResponse {
                candidates: candidates,
            }

        )
        )})
        .map_err(|e| GenericServiceError {
            error: GenericServiceErrorDetails::new_geocoding_error(e)
        })?
    )
}