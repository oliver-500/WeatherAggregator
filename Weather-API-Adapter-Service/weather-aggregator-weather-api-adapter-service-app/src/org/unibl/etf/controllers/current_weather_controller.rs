use actix_web::{web, HttpResponse, Responder};

use actix_web_validator::Query;
use reqwest::Client;
use crate::org::unibl::etf::configuration::settings::{Settings};
use crate::org::unibl::etf::controllers::errors::generic_service_error::{GenericServiceError, GenericServiceErrorDetails};

use crate::org::unibl::etf::model::requests::current_weather_request::{CurrentWeatherRequest};
use crate::org::unibl::etf::model::responses::uniform_current_weather_response::UniformCurrentWeatherResponse;
use crate::org::unibl::etf::services::current_weather_service::CurrentWeatherService;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/current_weather").route(web::get().to(get_current_weather_data)));
}

async fn get_current_weather_data(
    current_weather_service: web::Data<CurrentWeatherService>,
    query: Query<CurrentWeatherRequest>,
    http_client: web::Data<Client>,
    settings: web::Data<Settings>,
) -> Result<impl Responder, GenericServiceError> {

    println!("lokacija: {:?}", query.as_ref());
    let res = current_weather_service
        .get_current_weather_by_coordinates_or_location_name(
            query.as_ref(),
            http_client.get_ref(),
            &settings.provider
        )
        .await
        .map_err(|e| GenericServiceError {
            error: GenericServiceErrorDetails::new_adapter_error(&settings.provider.name, e)
        })?;

    Ok(UniformCurrentWeatherResponse::try_from(res)
        .and_then(|weather_data| Ok(HttpResponse::Ok().json(weather_data)))
        .map_err(|e| GenericServiceError {
            error: GenericServiceErrorDetails::new_adapter_error(&settings.provider.name, e)
        })?
    )

}