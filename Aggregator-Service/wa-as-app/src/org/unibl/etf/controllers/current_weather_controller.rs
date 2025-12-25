
use actix_web::{web, HttpResponse, Responder};
use actix_web_validator::Query;
use reqwest::Client;
use crate::org::unibl::etf::configuration;
use crate::org::unibl::etf::controllers::errors::{GenericServiceError, GenericServiceErrorDetails};
use crate::org::unibl::etf::model::requests::upstream_current_weather_request::UpstreamCurrentWeatherRequest;
use crate::org::unibl::etf::services::current_weather_service::CurrentWeatherService;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/currentweather").route(web::get().to(get_current_weather_data)));
}

async fn get_current_weather_data(
    query: Query<UpstreamCurrentWeatherRequest>,
    http_client: web::Data<Client>,
    configuration: web::Data<configuration::Settings>,
    current_weather_service: web::Data<CurrentWeatherService>
) -> Result<impl Responder, GenericServiceError> {
    println!("{:?}", query.location);

    Ok(current_weather_service.get_current_weather_data(
        query.as_ref(),
        http_client.as_ref(),
        configuration.as_ref()
    ).await
           .and_then(|current_weather_data| Ok(HttpResponse::Ok().json(current_weather_data)))
           .map_err(|e| GenericServiceError {
               error: GenericServiceErrorDetails::new_aggregator_error(e)
           })?
    )
}