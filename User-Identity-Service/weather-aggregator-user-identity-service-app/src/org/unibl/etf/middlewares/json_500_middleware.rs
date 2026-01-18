use actix_web::{
    body::{EitherBody, MessageBody},
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header::CONTENT_TYPE,
    http::StatusCode,
    Error, HttpResponse,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use chrono::{ Utc};
use std::rc::Rc;
use crate::org::unibl::etf::controllers::errors::{GenericServiceError, GenericServiceErrorDetails};
use crate::org::unibl::etf::model::errors::user_identity_service_error::{UserIdentityServiceError};

pub struct Json500Middleware;

impl<S, B> Transform<S, ServiceRequest> for Json500Middleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = Json500MiddlewareInner<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(Json500MiddlewareInner {
            service: Rc::new(service),
        })
    }
}

pub struct Json500MiddlewareInner<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for Json500MiddlewareInner<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            let res = svc.call(req).await?;

            // Only intercept 500
            if res.status() != StatusCode::INTERNAL_SERVER_ERROR {
                return Ok(res.map_into_left_body());
            }

            let is_json = res
                .headers()
                .get(CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .map(|v| v.starts_with("application/json"))
                .unwrap_or(false);

            if is_json {
                return Ok(res.map_into_left_body());
            }

            let generic_error = GenericServiceError {
                error: GenericServiceErrorDetails {
                    code: UserIdentityServiceError::ServerError(None),
                    code_numeric: 500,
                    message: "Internal server error".to_string(),
                    timestamp: Utc::now(),
                },
            };

            let (req, _) = res.into_parts();

            let json_response = HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(generic_error)
                .map_into_right_body();

            Ok(ServiceResponse::new(req, json_response))
        })
    }
}