use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use std::{
    future::{ready, Ready},
    pin::Pin,
    sync::{
        Arc,

    },
    task::{Context, Poll},
};
use actix_web::body::EitherBody;
use chrono::Utc;
use crate::org::unibl::etf::controllers::errors::{GenericServiceError, GenericServiceErrorDetails};
use crate::org::unibl::etf::model::errors::user_identity_service_error::UserPreferencesServiceError;
use crate::org::unibl::etf::services::jwt_service::JwtService;

pub struct JwtMiddleware {
    pub jwt_service: Arc<JwtService>,
}

impl JwtMiddleware {
    pub fn new_with_public_key(
        jwt_service: Arc<JwtService>
    ) -> Self {
        Self {
            jwt_service
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = JwtMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareMiddleware {
            service,
            jwt_service: self.jwt_service.clone(),
        }))
    }


}

pub struct JwtMiddlewareMiddleware<S> {
    service: S,
    jwt_service: Arc<JwtService>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn futures::Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("provjera");
        let cookie = req.cookie("jwt");

        match cookie {
            Some(cookie) => {
                match self.jwt_service.validate_token(cookie.value()) {
                    Ok(_claims) => {
                        let fut = self.service.call(req);
                        println!("Zahtjev prosao");
                        return Box::pin(async move {
                            let res = fut.await?;
                            Ok(res.map_into_left_body())
                        });
                    },
                    Err(error) => {
                        println!("Zahtjev prekinut1");
                        match *error.kind() {
                            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                                //if token is expired get sub from expired token
                                let err = GenericServiceError {
                                    error: GenericServiceErrorDetails {
                                        code: UserPreferencesServiceError::Unauthorized(Some("Credentials expired. Please log in again.".to_string())),
                                        code_numeric: 401,
                                        message: "".to_string(),
                                        timestamp: Utc::now(),
                                    }
                                };
                                // let resp = HttpResponse::InternalServerError()
                                //     .body("App currently unavailable. Try again later.")
                                //     .map_into_right_body();

                                return Box::pin(ready(Err(err.into())));

                            },
                            jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                                tracing::error!("Token signature is wrong! Possible tampered token.");
                                let err = GenericServiceError {
                                    error: GenericServiceErrorDetails {
                                        code: UserPreferencesServiceError::Unauthorized(Some("Credentials invalid.".to_string())),
                                        code_numeric: 401,
                                        message: "".to_string(),
                                        timestamp: Utc::now(),
                                    }
                                };
                                // let resp = HttpResponse::InternalServerError()
                                //     .body("App currently unavailable. Try again later.")
                                //     .map_into_right_body();

                                return Box::pin(ready(Err(err.into())));
                            },
                            _ => {
                                let err = GenericServiceError {
                                    error: GenericServiceErrorDetails {
                                        code: UserPreferencesServiceError::ServerError(None),
                                        code_numeric: 401,
                                        message: "".to_string(),
                                        timestamp: Utc::now(),
                                    }
                                };
                                // let resp = HttpResponse::InternalServerError()
                                //     .body("App currently unavailable. Try again later.")
                                //     .map_into_right_body();

                                return Box::pin(ready(Err(err.into())));
                            }
                        };
                    }
                }

            },
            None => {
                println!("Zahtjev prekinut");

                //let (req, _) = req.into_parts();
                let err = GenericServiceError {
                    error: GenericServiceErrorDetails {
                        code: UserPreferencesServiceError::Unauthorized(Some("Credentials not present".to_string())),
                        code_numeric: 401,
                        message: "".to_string(),
                        timestamp: Utc::now(),
                    }
                };
                // let resp = HttpResponse::InternalServerError()
                //     .body("App currently unavailable. Try again later.")
                //     .map_into_right_body();

                return Box::pin(ready(Err(err.into())));

                //return Box::pin(async { Ok(actix_web::dev::ServiceResponse::new(req, resp)) });
            }
        }



    }
}
