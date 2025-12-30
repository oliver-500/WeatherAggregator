use actix_web::{web, HttpResponse, Responder};
use actix_web_validator::Query;
use crate::org::unibl::etf::controllers::errors::generic_service_error::{GenericServiceError, GenericServiceErrorDetails};
use crate::org::unibl::etf::model::requests::current_weather_cache_request::{CurrentWeatherCacheRequest};
use crate::org::unibl::etf::model::responses::current_weather_cache_response::CurrentWeatherCacheResponse;
use crate::org::unibl::etf::services::cache_service::{CacheService};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/current_weather")
            .route(web::get().to(get_current_weather_cached_results))
            .route(web::put().to(store_current_weather_cache_result))
    );
}

async fn get_current_weather_cached_results(
    cache_service: web::Data<CacheService>,
    query: Query<CurrentWeatherCacheRequest>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> Result<impl Responder, GenericServiceError> {
    Ok(cache_service
        .get_current_weather_cached_result(
            query.as_ref(),
            redis_pool.get_ref(),
        )
        .await
        .and_then(|res| {
            println!("{:?}", res);
            Ok(HttpResponse::Ok().json(res))
        })
        .map_err(|e| GenericServiceError {
            error: GenericServiceErrorDetails::new_cache_error(e)
        })?
    )
}

async fn store_current_weather_cache_result(
    cache_service: web::Data<CacheService>,
    query: Query<CurrentWeatherCacheResponse>, //da ne bude response
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> Result<impl Responder, GenericServiceError> {
    Ok(cache_service
        .store_current_weather_result_as_cache(
            query.as_ref(),
            redis_pool.get_ref(),
        )
        .await
        .and_then(|res| {
            println!("{:?}", res);
            Ok(HttpResponse::Ok().json(res))
        })
        .map_err(|e| GenericServiceError {
            error: GenericServiceErrorDetails::new_cache_error(e)
        })?
    )
}