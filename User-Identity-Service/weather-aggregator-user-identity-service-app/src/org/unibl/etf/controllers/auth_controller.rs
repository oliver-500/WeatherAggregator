
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_web::cookie::Cookie;
use actix_web::cookie::time::Duration;
use crate::org::unibl::etf::controllers::errors::{GenericServiceError, GenericServiceErrorDetails};
use crate::org::unibl::etf::model::errors::user_identity_service_error::UserIdentityServiceError;
use crate::org::unibl::etf::model::requests::register_standard_user_request::RegisterStandardUserRequest;
use crate::org::unibl::etf::services::auth_service::AuthService;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/auth/anonymous").route(web::get().to(register_anonymous_user)))
        .service(web::resource("/auth/register").route(web::post().to(register_standard_user)))
        // .service(web::resource("/auth/login").route(web::post().to(authenticate_standard_user)))
        .service(web::resource("/auth/refresh").route(web::get().to(refresh_access_token)));
        // .service(web::resource("/auth/logout").route(web::post().to(logout_user)));
}


#[tracing::instrument(name = "Auth controller - register standard user function",
    skip(auth_service))]
async fn register_standard_user(
    request_body: web::Json<RegisterStandardUserRequest>,
    request: HttpRequest,
    auth_service: web::Data<AuthService>,
) -> Result<impl Responder, GenericServiceError> {
    // let jwt: Option<String> = if request_body.use_previously_saved_data == true {
    //     match retrieve_jwt_from_cookie(&request) {
    //         Err(e) => {
    //             tracing::error!("Use previously saved data requested but cookie not found with error: {:?}", e);
    //             None
    //         },
    //         Ok(jwt) => Some(jwt)
    //     }
    // };

    // match auth_service.register_standard_user(request_body.into_inner(), jwt) {
    //
    // }

    let err: GenericServiceError = GenericServiceError {
        error: GenericServiceErrorDetails::new_user_identity_service_error(UserIdentityServiceError::ServerError(None))
    };
    return Err::<HttpResponse, GenericServiceError>(err);

}


#[tracing::instrument(name = "Auth controller - retrieve jwt from cookie function",)]
fn retrieve_jwt_from_cookie(req: &HttpRequest) -> Result<String, UserIdentityServiceError> {
    match req.cookie("jwt") {
        Some(cookie) => {
            Ok(cookie.clone().value().to_string())
        }
        None => {
            //if not present return cookie not found
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
    //retrieve jwt token from cookie
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
            let jwt_cookie = Cookie::build("jwt", token)
                .path("/")
                .http_only(true)    // Prevents JS access (XSS protection)
                .secure(true)
                .max_age(Duration::days(400))// Ensures cookie is sent over HTTPS only
                .same_site(actix_web::cookie::SameSite::Strict)
                .finish();

            return Ok(HttpResponse::Ok()
                .cookie(jwt_cookie)
                .finish())
        }
        Err(e) => {
            let err = GenericServiceError {
                error: GenericServiceErrorDetails::new_user_identity_service_error(e)
            };
            return Err(err);
        }
    }
}

#[tracing::instrument(name = "Auth controller - register anonymous user function",
    skip(auth_service))]
async fn register_anonymous_user(
    auth_service: web::Data<AuthService>
) -> Result<impl Responder, GenericServiceError> {
    Ok(auth_service.register_anonymous_user().await
           .and_then(|cookie_value| {
               let jwt_cookie = Cookie::build("jwt", cookie_value)
                   .path("/")
                   .http_only(true)    // Prevents JS access (XSS protection)
                   .secure(true)
                   .max_age(Duration::days(400))// Ensures cookie is sent over HTTPS only
                   .same_site(actix_web::cookie::SameSite::Strict)
                   .finish();

               Ok(HttpResponse::Ok()
                   .cookie(jwt_cookie)
                   .finish())
           })
           .map_err(|e| {
               tracing::error!("Was not able to register anonymous user with error: {:?}", e);
               GenericServiceError {
                   error: GenericServiceErrorDetails::new_user_identity_service_error(e)
               }
           })?
    )
}

