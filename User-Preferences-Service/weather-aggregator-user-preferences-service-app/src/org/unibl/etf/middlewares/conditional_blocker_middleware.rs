use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse,
};
use std::{
    future::{ready, Ready},
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    task::{Context, Poll},
};
use actix_web::body::EitherBody;
use chrono::Utc;
use crate::org::unibl::etf::controllers::errors::{GenericServiceError, GenericServiceErrorDetails};
use crate::org::unibl::etf::model::errors::user_identity_service_error::{ UserPreferencesServiceError};

pub struct ConditionalBlocker {
    is_db_up: Arc<AtomicBool>,
    is_broker_up: Arc<AtomicBool>,
}

impl ConditionalBlocker {
    pub fn new(
        is_db_up: Arc<AtomicBool>,
        is_broker_up: Arc<AtomicBool>
    ) -> Self {
        Self { is_db_up, is_broker_up }
    }
}

impl<S, B> Transform<S, ServiceRequest> for ConditionalBlocker
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = ConditionalBlockerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ConditionalBlockerMiddleware {
            service,
            is_db_up: self.is_db_up.clone(),
            is_broker_up: self.is_broker_up.clone(),
        }))
    }
}

pub struct ConditionalBlockerMiddleware<S> {
    service: S,
    is_db_up: Arc<AtomicBool>,
    is_broker_up: Arc<AtomicBool>,
}

impl<S, B> Service<ServiceRequest> for ConditionalBlockerMiddleware<S>
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
        let path = req.path().to_owned();
        println!("provjera");

        if !self.is_db_up.load(Ordering::Relaxed) {
            let err = GenericServiceError {
                error: GenericServiceErrorDetails {
                    code: UserPreferencesServiceError::ApplicationCurrentlyUnavailable,
                    code_numeric: 500,
                    message: "App currently unavailable.".to_string(),
                    timestamp: Utc::now(),
                },
            };
            return Box::pin(ready(Err(err.into())));
        };


        let fut = self.service.call(req);
        return Box::pin(async move {
            let res = fut.await?;
            Ok(res.map_into_left_body())
        });


    }
}
