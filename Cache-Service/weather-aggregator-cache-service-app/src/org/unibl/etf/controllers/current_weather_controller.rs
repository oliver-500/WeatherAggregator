use actix_web::{web, HttpResponse, Responder};
use actix_web_validator::Query;
use crate::org::unibl::etf::controllers::errors::generic_service_error::{GenericServiceError, GenericServiceErrorDetails};
use crate::org::unibl::etf::model::requests::retrieve_current_weather_cache_request::{RetrieveCurrentWeatherCacheRequest};
use crate::org::unibl::etf::model::requests::store_current_weather_data_request::StoreCurrentWeatherDataRequest;
use crate::org::unibl::etf::services::cache_service::{CacheService};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/current_weather_by_coordinates")
            .route(web::get().to(get_current_weather_cache_data_by_coordinates))
        )
        .service(web::resource("/current_weather_by_location")
                     .route(web::get().to(get_current_weather_cache_data_by_location))
        )
        .service(web::resource("/current_weather")
            .route(web::put().to(store_current_weather))
        );
}

#[tracing::instrument(name = "Get Current Weather Data Cache Controller",
    skip(cache_service, redis_pool))]
async fn get_current_weather_cache_data_by_coordinates(
    cache_service: web::Data<CacheService>,
    query: Query<RetrieveCurrentWeatherCacheRequest>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> Result<impl Responder, GenericServiceError> {
    Ok(cache_service
        .get_current_weather_cache_data_by_coordinates(
            query.as_ref(),
            redis_pool.get_ref(),
        )
        .await
        .and_then(|res| {
            Ok(HttpResponse::Ok().json(res))
        })
        .map_err(|e| {
            tracing::error!("Was not able to get current weather cache data with error: {:?}", e);
            GenericServiceError {
                error: GenericServiceErrorDetails::new_cache_error(e)
            }
        })?
    )
}


#[tracing::instrument(name = "Get Current Weather Data Cache Controller",
    skip(cache_service, redis_pool))]
async fn get_current_weather_cache_data_by_location(
    cache_service: web::Data<CacheService>,
    query: Query<RetrieveCurrentWeatherCacheRequest>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> Result<impl Responder, GenericServiceError> {
    println!("joj");
    Ok(cache_service
        .get_current_weather_cache_data_by_location(
            query.as_ref(),
            redis_pool.get_ref(),
        )
        .await
        .and_then(|res| {
            Ok(HttpResponse::Ok().json(res))
        })
        .map_err(|e| {
            tracing::error!("Was not able to get current weather cache data with error: {:?}", e);
            GenericServiceError {
                error: GenericServiceErrorDetails::new_cache_error(e)
            }
        })?
    )
}


#[tracing::instrument(name = "Store Current Weather Data Cache Controller",
    skip(cache_service, redis_pool))]
async fn store_current_weather(
    cache_service: web::Data<CacheService>,
    req: web::Json<StoreCurrentWeatherDataRequest>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> Result<impl Responder, GenericServiceError> {
    Ok(cache_service
        .store_current_weather_result_as_cache(
            &req.into_inner(),
            redis_pool.get_ref(),
        )
        .await
        .and_then(|res| {
            Ok(HttpResponse::Ok().json(res))
        })
        .map_err(|e| {
            tracing::error!("Was not able to store current weather cache data with error: {:?}", e);
            GenericServiceError {
                error: GenericServiceErrorDetails::new_cache_error(e)
            }
        })?
    )
}