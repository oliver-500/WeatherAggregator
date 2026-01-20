use std::str::FromStr;
use secrecy::{ExposeSecret, SecretString};
use tracing::{Instrument, Span};
use uuid::Uuid;

use crate::org::unibl::etf::model::domain::entities::user_entity::user_password::{UserPassword};
use crate::org::unibl::etf::model::domain::entities::user_entity::UserEntity;

use crate::org::unibl::etf::model::domain::messages::anonymous_user_registered::AnonymousUserRegistered;
use crate::org::unibl::etf::model::domain::messages::standard_user_registered::StandardUserRegistered;
use crate::org::unibl::etf::model::errors::user_identity_service_error::UserIdentityServiceError;
use crate::org::unibl::etf::model::requests::login_standard_user_request::LoginStandardUserRequest;
use crate::org::unibl::etf::model::requests::register_standard_user_request::RegisterStandardUserRequest;
use crate::org::unibl::etf::model::responses::user_registered_response::UserRegisteredResponse;
use crate::org::unibl::etf::model::user_type::UserType;
use crate::org::unibl::etf::publishers::user_publisher::UserPublisher;
use crate::org::unibl::etf::repositories::user_identity_repository::UserIdentityRepository;
use crate::org::unibl::etf::services::jwt_service::JwtService;

#[derive(Debug)]
pub struct AuthService {
    pub jwt_service: JwtService,
    pub user_identity_repository: UserIdentityRepository,
    pub user_publisher: UserPublisher,
}


impl AuthService {
    // fn new() -> Self {
    //     Self {
    //         jwt_service: JwtService::default(),
    //         user_identity_repository: UserIdentityRepository::default()
    //     }
    // }

    #[tracing::instrument(name = "Auth service - authenticate standard user function", skip(
        self
    ))]
    pub async fn authenticate_standard_user(
        &self,
        req: LoginStandardUserRequest
    ) -> Result<String, UserIdentityServiceError> {
        let user = match self
            .user_identity_repository
            .get_user_by_email(&req.email)
            .await {
            Ok(user) => {
                user
            },
            Err(error) => {
                return Err(UserIdentityServiceError::DatabaseError(Some(format!("Error while retrieving user by email. {}", error.to_string()))));
            }
        };

        match UserPassword::verify_password(req.password.expose_secret(), user.password.as_ref()) {
            Err(error) => {
                return Err(UserIdentityServiceError::ServerError(Some(format!("Error while verifying password. {}", error.to_string()))));
            },
            Ok(validated) => {
                if !validated {
                    return Err(UserIdentityServiceError::UserError(Some(format!("Invalid password."))));
                }
            }
        }

        match self.jwt_service.generate_token(
            user.id.to_string().as_str(),
            UserType::STANDARD
        ) {
            Err(error) => {
                Err(UserIdentityServiceError::ServerError(Some(format!("Error while generating token. {}", error.to_string()))))
            },
            Ok(token) => {
                Ok(token)
            }
        }
    }

    #[tracing::instrument(name = "Auth service - register standard user function", skip(
        self
    ))]
    pub async fn register_standard_user(
        &self,
        request: &RegisterStandardUserRequest,
        jwt: Option<String>
    ) -> Result<UserRegisteredResponse, UserIdentityServiceError> {
        let mut registry_entity: UserEntity = match request.try_into() {
            Ok(r) => r,
            Err(e) => return Err(UserIdentityServiceError::RequestValidationError(Some(e.to_string()))),
        };

        let password =  match UserPassword::hash_password(registry_entity.password.as_ref()) {
            Ok(pw) => pw,
            Err(e) => return Err(UserIdentityServiceError::ServerError(Some(format!("Failed to compute hash for password: {}:", e.to_string()))))
        };

        registry_entity.password = UserPassword(SecretString::from(password));

        let mut old_id: Option<Uuid> = None;

        if jwt.is_some() {
            let user_id = match self.jwt_service.get_claims_from_token(&jwt.unwrap()) {
                Ok(claims) => {
                    claims.sub
                },
                Err(e) => {
                    return Err(UserIdentityServiceError::TamperedJwtTokenError(Some(e.to_string())))
                }
            };

            old_id = match Uuid::from_str(user_id.as_str()) {
                Ok(id) => Some(id),
                Err(e) => return Err(UserIdentityServiceError::ServerError(Some(e.to_string()))),
            };
        }

        let res = match self
            .user_identity_repository
            .insert_user(&mut registry_entity)
            .await {
            Ok(r) => r,
            Err(db_err) => {
                if let Some(db_err) = db_err.as_database_error() {
                    // "23505" is the Postgres code for unique_violation
                    if db_err.code() == Some(std::borrow::Cow::Borrowed("23505")) {
                        return Err(UserIdentityServiceError::UserError(Some("Email already taken. Try another one.".to_string())));
                    }
                }
                return Err(UserIdentityServiceError::DatabaseError(Some(db_err.to_string())))
            },
        };

        let res_clone = res.clone();
        let user_publisher_clone = self.user_publisher.clone();

        tokio::spawn( async move {
            let mut event: StandardUserRegistered = (&res_clone).into();

            if let Some(id) = old_id {
                event.old_id = Some(id);
            }

            user_publisher_clone
                .publish_standard_user_registered_event(event)
                .await
                .map_err(|e| {
                    UserIdentityServiceError::ServerError(Some(e.to_string()))
                }).unwrap_or_else(|e| {
                    tracing::error!("Failed to publish message: Error: {:?}", e);
            });
        }.instrument(Span::current()));

        let res = UserRegisteredResponse {
            id: res.id,
        };
        Ok(res)
    }

    #[tracing::instrument(name = "Auth service - register anonymous user function", skip(
        self
    ))]
    pub async fn register_anonymous_user(&self) -> Result<(String, UserRegisteredResponse), UserIdentityServiceError> {
        let user_id = Uuid::new_v4();
        let user = AnonymousUserRegistered {
            user_type: UserType::GUEST,
            id: user_id.clone(),
        };

        let user_publisher_clone = self.user_publisher.clone();

        tokio::spawn( async move {
            match user_publisher_clone
                .publish_anonymous_user_registered_event(user)
                .await {
                    Ok(_) => {},
                    Err(e) => {
                    tracing::error!("Failed to publish message: Error: {:?}", e);
                }
            }
        }.instrument(Span::current())
        );

        match self.jwt_service.generate_token(&user_id.to_string(), UserType::GUEST) {
            Ok(token) => {
                let res = UserRegisteredResponse {
                    id: user_id,
                };
                Ok((token, res))
            },
            Err(error) => {
                Err(UserIdentityServiceError::ServerError(Some(error.to_string())))
            }
        }
    }

    #[tracing::instrument(name = "Auth service - refresh access token function", skip(

    ))]
    pub async fn refresh_access_token(&self, token: &str) -> Result<String, UserIdentityServiceError> {

        match self.jwt_service.validate_token(token) {
            Ok(_claims) => return Ok(token.to_string()),
            Err(error) => {
                match *error.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        //if token is expired get sub from expired token
                        let claims = self.jwt_service.get_claims_from_token(token);

                        if let Err(e) = claims {
                            tracing::error!("Could not extract sub from jwt claims: {}.", e);
                            return Err(UserIdentityServiceError::TamperedJwtTokenError(None))
                        }

                        let claims = claims.unwrap();

                        //generate new token with sub
                        match self.jwt_service.generate_token(&claims.sub.to_string(), claims.user_type) {
                            Ok(token) => {

                                return Ok(token)
                            },
                            Err(error) => {
                                return Err(UserIdentityServiceError::ServerError(Some(error.to_string())));
                            }
                        }

                    },
                    jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                        tracing::error!("Token signature is wrong! Possible tampered token.");
                        return Err(UserIdentityServiceError::TamperedJwtTokenError(None))
                    },
                    _ => {
                        tracing::error!("Token validation failed!");
                        return Err(UserIdentityServiceError::ServerError(None))
                    }
                };
            }
        }

    }



}


// impl Default for AuthService {
//     fn default() -> Self {
//         Self::new()
//     }
// }
