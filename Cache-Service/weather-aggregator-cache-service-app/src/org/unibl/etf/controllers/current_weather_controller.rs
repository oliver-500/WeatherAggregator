use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_web_validator::Query;
use opentelemetry::trace::TraceContextExt;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use crate::org::unibl::etf::controllers::errors::generic_service_error::{GenericServiceError, GenericServiceErrorDetails};
use crate::org::unibl::etf::model::requests::current_weather_cache_request::{CurrentWeatherCacheRequest};
use crate::org::unibl::etf::model::requests::store_current_weather_data_request::StoreCurrentWeatherDataRequest;
use crate::org::unibl::etf::services::cache_service::{CacheService};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/current_weather")
            .route(web::get().to(get_current_weather_cached_results))
            .route(web::put().to(store_current_weather_cache_result))
    );
}

#[tracing::instrument(name = "Get Current Weather Data Cache Controller",
    skip(cache_service, redis_pool))]
async fn get_current_weather_cached_results(
    cache_service: web::Data<CacheService>,
    query: Query<CurrentWeatherCacheRequest>,
    redis_pool: web::Data<deadpool_redis::Pool>,
    req: HttpRequest,
) -> Result<impl Responder, GenericServiceError> {

    if let Some(val) = req.headers().get("traceparent") {
        println!("Received traceparent: {:?}", val);
    } else {
        println!("CRITICAL: No traceparent header found in Service B!");
    }



    let context = tracing::Span::current().context();
    let context_span = context.span();
    let span_context = context_span.span_context();

    if span_context.is_valid() {
        println!("Service Trace ID: {}", span_context.trace_id());
    } else {
        println!("No valid Trace ID found in current span!");
    }

    Ok(cache_service
        .get_current_weather_cached_result(
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
async fn store_current_weather_cache_result(
    cache_service: web::Data<CacheService>,
    req: web::Json<StoreCurrentWeatherDataRequest>, //da ne bude response
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