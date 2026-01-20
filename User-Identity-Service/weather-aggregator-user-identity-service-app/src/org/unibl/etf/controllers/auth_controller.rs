
use actix_web::{cookie, web, HttpRequest, HttpResponse, Responder};
use actix_web::cookie::Cookie;
use actix_web::cookie::time::Duration;
use crate::org::unibl::etf::controllers::errors::{GenericServiceError, GenericServiceErrorDetails};
use crate::org::unibl::etf::model::errors::user_identity_service_error::UserIdentityServiceError;
use crate::org::unibl::etf::model::requests::login_standard_user_request::LoginStandardUserRequest;
use crate::org::unibl::etf::model::requests::register_standard_user_request::RegisterStandardUserRequest;
use crate::org::unibl::etf::services::auth_service::AuthService;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/auth/anonymous").route(web::get().to(register_anonymous_user)))
        .service(web::resource("/auth/register").route(web::post().to(register_standard_user)))
        .service(web::resource("/auth/login").route(web::post().to(authenticate_standard_user)))
        .service(web::resource("/auth/refresh").route(web::get().to(refresh_access_token)));
        // .service(web::resource("/auth/logout").route(web::post().to(logout_user)));
}

fn build_jwt_cookie_with_token(token: String) -> cookie::Cookie<'static> {
    Cookie::build("jwt", token)
        .path("/")
        .http_only(true)    // Prevents JS access (XSS protection)
        .secure(true)
        .max_age(Duration::days(400))// Ensures cookie is sent over HTTPS only
        .same_site(actix_web::cookie::SameSite::Strict)
        .finish()
}

#[tracing::instrument(name = "Auth controller - authenticate standard user function",
    skip(auth_service))]
async fn authenticate_standard_user(
    auth_service: web::Data<AuthService>,
    request_body: web::Json<LoginStandardUserRequest>,
) -> Result<impl Responder, GenericServiceError> {
    match auth_service.authenticate_standard_user(request_body.into_inner()).await {
        Ok(token) => {
            Ok(HttpResponse::Ok()
                .cookie(build_jwt_cookie_with_token(token))
                .finish())
        },
        Err(err) => {
            let err = GenericServiceError {
                error: GenericServiceErrorDetails::new_user_identity_service_error(err)
            };
            Err(err)
        }
    }
}

#[tracing::instrument(name = "Auth controller - register standard user function",
    skip(auth_service))]
async fn register_standard_user(
    request_body: web::Json<RegisterStandardUserRequest>,
    request: HttpRequest,
    auth_service: web::Data<AuthService>,
) -> Result<impl Responder, GenericServiceError> {
    let jwt: Option<String> = if request_body.use_previously_saved_data == true {
        match retrieve_jwt_from_cookie(&request) {
            Err(e) => {
                tracing::warn!("Use previously saved data requested but cookie not found with error: {:?}", e);
                None
            },
            Ok(jwt) => Some(jwt)
        }
    } else {
        None
    };

    match auth_service.register_standard_user(&request_body.into_inner(), jwt).await {
        Ok(res) => {
            return Ok(HttpResponse::Ok().json(res));
        },
        Err(e) => {
            let err = GenericServiceError {
                error: GenericServiceErrorDetails::new_user_identity_service_error(e)
            };
            return Err(err);
        }
    }
}


#[tracing::instrument(name = "Auth controller - retrieve jwt from cookie function",)]
fn retrieve_jwt_from_cookie(req: &HttpRequest) -> Result<String, UserIdentityServiceError> {
    match req.cookie("jwt") {
        Some(cookie) => {
            Ok(cookie.clone().value().to_string())
        }
        None => {
            return Err(UserIdentityServiceError::JwtCookieNotFoundError(None));
        }
    }
}

#[tracing::instrument(name = "Auth controller - refresh access token function",
    skip(auth_service))]
async fn refresh_access_token(
    req: HttpRequest,
    auth_service: web::Data<AuthService>,
) -> Result<impl Responder, GenericServiceError> {
    let jwt = match retrieve_jwt_from_cookie(&req) {
        Err(e) => {
            let err = GenericServiceError {
                error: GenericServiceErrorDetails::new_user_identity_service_error(e)
            };
            return Err(err);
        },
        Ok(jwt) => jwt
    };

    match auth_service.refresh_access_token(&jwt).await {
        Ok(token) => {
            Ok(HttpResponse::Ok()
                .cookie(build_jwt_cookie_with_token(token))
                .finish())
        }
        Err(e) => {
            let err = GenericServiceError {
                error: GenericServiceErrorDetails::new_user_identity_service_error(e)
            };
            Err(err)
        }
    }
}

#[tracing::instrument(name = "Auth controller - register anonymous user function",
    skip(auth_service))]
async fn register_anonymous_user(
    auth_service: web::Data<AuthService>
) -> Result<impl Responder, GenericServiceError> {
    Ok(auth_service.register_anonymous_user().await
           .and_then(|(token, res)| {
               Ok(HttpResponse::Ok()
                   .cookie(build_jwt_cookie_with_token(token))
                   .json(res))
           })
           .map_err(|e| {
               tracing::error!("Was not able to register anonymous user with error: {:?}", e);
               GenericServiceError {
                   error: GenericServiceErrorDetails::new_user_identity_service_error(e)
               }
           })?
    )
}

