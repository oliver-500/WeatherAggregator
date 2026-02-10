
use actix_web::{cookie, web, HttpRequest, HttpResponse, Responder};
use actix_web::cookie::Cookie;
use actix_web::cookie::time::Duration;
use crate::org::unibl::etf::controllers::errors::{GenericServiceError};
use crate::org::unibl::etf::model::errors::user_identity_service_error::UserIdentityServiceError;
use crate::org::unibl::etf::model::requests::login_standard_user_request::LoginStandardUserRequest;
use crate::org::unibl::etf::model::requests::register_standard_user_request::RegisterStandardUserRequest;
use crate::org::unibl::etf::services::auth_service::AuthService;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/auth/anonymous").route(web::get().to(register_anonymous_user)))
        .service(web::resource("/auth/refresh").route(web::get().to(refresh_access_token)))
        .service(web::resource("/auth/user_info").route(web::get().to(get_user_info)))
        .service(web::resource("/auth/register").route(web::post().to(register_standard_user)))
        .service(web::resource("/auth/login").route(web::post().to(authenticate_standard_user)))
        .service(web::resource("/auth/logout").route(web::post().to(logout_user)));
}

#[tracing::instrument(
    name = "Auth controller - register anonymous user function",
    skip(auth_service)
)]
async fn register_anonymous_user(
    auth_service: web::Data<AuthService>
) -> Result<impl Responder, GenericServiceError> {
    let res = auth_service.register_anonymous_user().await
        .and_then(|(access_token, refresh_token, res)| {
            Ok(HttpResponse::Ok()
                .cookie(build_cookie_with_token(access_token, "access_token"))
                .cookie(build_cookie_with_token(refresh_token, "refresh_token"))
                .json(res)
            )
        })?;

    Ok(res)
}


#[tracing::instrument(
    name = "Auth controller - refresh access token function",
    skip(auth_service)
)]
async fn refresh_access_token(
    req: HttpRequest,
    auth_service: web::Data<AuthService>,
) -> Result<impl Responder, GenericServiceError> {
    let access_token = retrieve_token_from_cookie(&req, "access_token")?;
    let refresh_token = retrieve_token_from_cookie(&req, "refresh_token")?;

    let res = auth_service
        .refresh_access_token(
            &access_token,
            &refresh_token
        )
        .await
        .and_then(|(access_token, refresh_token)| {
            Ok(HttpResponse::Ok()
                .cookie(build_cookie_with_token(access_token, "access_token"))
                .cookie(build_cookie_with_token(refresh_token, "refresh_token"))
                .finish()
            )
        })?;

    Ok(res)
}


#[tracing::instrument(
    name = "Auth controller - get user info function",
    skip(auth_service)
)]
async fn get_user_info(
    auth_service: web::Data<AuthService>,
    req: HttpRequest,
) -> Result<impl Responder, GenericServiceError> {
    let access_token = retrieve_token_from_cookie(&req, "access_token")?;

    let res = auth_service
        .get_user_info(access_token)
        .await
        .and_then(|res| {
            Ok(HttpResponse::Ok()
                .json(res)
            )
        })?;

    Ok(res)
}


#[tracing::instrument(
    name = "Auth controller - register standard user function",
    skip(auth_service)
)]
async fn register_standard_user(
    request_body: web::Json<RegisterStandardUserRequest>,
    request: HttpRequest,
    auth_service: web::Data<AuthService>,
) -> Result<impl Responder, GenericServiceError> {
    let jwt: Option<String> = if request_body.use_previously_saved_data == true {
        match retrieve_token_from_cookie(&request, "access_token") {
            Err(e) => {
                tracing::warn!("Use previously saved data requested but cookie not found with error: {:?}", e);
                None
            },
            Ok(jwt) => Some(jwt)
        }
    } else {
        None
    };

    let res = auth_service.register_standard_user(
        &request_body.into_inner(),
        jwt
    ).await
        .and_then(|(access_token, refresh_token, res)| {
            Ok(HttpResponse::Ok()
                .cookie(build_cookie_with_token(access_token, "access_token"))
                .cookie(build_cookie_with_token(refresh_token, "refresh_token"))
                .json(res)
            )
        })?;

    Ok(res)
}



#[tracing::instrument(
    name = "Auth controller - authenticate standard user function",
    skip(auth_service)
)]
async fn authenticate_standard_user(
    auth_service: web::Data<AuthService>,
    request_body: web::Json<LoginStandardUserRequest>,
) -> Result<impl Responder, GenericServiceError> {
    let res = auth_service.
        authenticate_standard_user(
            request_body.into_inner()
        ).await
        .and_then(|(access_token, refresh_token)| {
            Ok(HttpResponse::Ok()
                .cookie(build_cookie_with_token(access_token, "access_token"))
                .cookie(build_cookie_with_token(refresh_token, "refresh_token"))
                .finish()
            )
        })?;

    Ok(res)
}

#[tracing::instrument(
    name = "Auth controller - logout user function",
)]
async fn logout_user(
) -> Result<impl Responder, GenericServiceError> {
    Ok(HttpResponse::Ok()
        .cookie(build_logout_cookie("refresh_token"))
        .cookie(build_logout_cookie("access_token"))
        .finish()
    )
}



fn build_cookie_with_token(token: String, cookie_name: &str) -> cookie::Cookie<'static> {
    Cookie::build(cookie_name.to_owned(), token)
        .path("/")
        .http_only(true)    // Prevents JS access (XSS protection)
        .secure(false)
        .max_age(Duration::days(400))// Ensures cookie is sent over HTTPS only
        .same_site(cookie::SameSite::Strict)
        .finish()
}

pub fn build_logout_cookie(cookie_name: &str) -> Cookie<'static> {
    Cookie::build(cookie_name.to_owned(), "")
        .path("/")
        .http_only(true)
        .secure(false)
        // Setting Max-Age to 0 tells the browser to delete it immediately
        .max_age(Duration::ZERO)
        // Setting an expiration in the past is a fallback for older browsers
        .expires(actix_web::cookie::time::OffsetDateTime::UNIX_EPOCH)
        .same_site(cookie::SameSite::Strict)
        .finish()
}

#[tracing::instrument(name = "Auth controller - retrieve token from cookie function",)]
fn retrieve_token_from_cookie(req: &HttpRequest, cookie_name: &str) -> Result<String, UserIdentityServiceError> {
    match req.cookie(cookie_name) {
        Some(cookie) => {
            Ok(cookie.clone().value().to_string())
        }
        None => {
            return Err(UserIdentityServiceError::JwtCookieNotFoundError(Some("JWT access token not found in cookies.".to_string())));
        }
    }
}



