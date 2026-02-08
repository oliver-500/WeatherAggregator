
use std::str::FromStr;
use secrecy::{ExposeSecret, SecretString};
use tracing::{Instrument, Span};
use uuid::Uuid;
use crate::org::unibl::etf::jwt::token_type::TokenType;
use crate::org::unibl::etf::model::domain::entities::refresh_token_entity::RefreshTokenEntity;
use crate::org::unibl::etf::model::domain::entities::user_entity::refresh_token::RefreshToken;
use crate::org::unibl::etf::model::domain::entities::user_entity::user_password::{UserPassword};
use crate::org::unibl::etf::model::domain::entities::user_entity::UserEntity;

use crate::org::unibl::etf::model::domain::messages::anonymous_user_registered::AnonymousUserRegistered;
use crate::org::unibl::etf::model::domain::messages::standard_user_registered::StandardUserRegistered;
use crate::org::unibl::etf::model::errors::user_identity_service_error::UserIdentityServiceError;
use crate::org::unibl::etf::model::requests::login_standard_user_request::LoginStandardUserRequest;
use crate::org::unibl::etf::model::requests::register_standard_user_request::RegisterStandardUserRequest;
use crate::org::unibl::etf::model::responses::user_info_response::UserInfoResponse;
use crate::org::unibl::etf::model::responses::user_registered_response::UserRegisteredResponse;
use crate::org::unibl::etf::model::user_type::UserType;
use crate::org::unibl::etf::publishers::user_publisher::UserPublisher;
use crate::org::unibl::etf::repositories::refresh_token_repository::RefreshTokenRepository;
use crate::org::unibl::etf::repositories::user_identity_repository::UserIdentityRepository;
use crate::org::unibl::etf::services::jwt_service::JwtService;

#[derive(Debug)]
pub struct AuthService {
    pub jwt_service: JwtService,
    pub user_identity_repository: UserIdentityRepository,
    pub user_publisher: UserPublisher,
    pub refresh_token_repository: RefreshTokenRepository,
}


impl AuthService {

    #[tracing::instrument(
        name = "Auth service - register anonymous user function",
        skip(self)
    )]
    pub async fn register_anonymous_user(
        &self
    ) -> Result<(String, String, UserRegisteredResponse), UserIdentityServiceError> {
        let user_id = Uuid::new_v4();

        let refresh_token = self.jwt_service.generate_token(
            user_id.clone().to_string().as_str(),
            UserType::GUEST,
            TokenType::REFRESH
        ).map_err(|error| return UserIdentityServiceError::ServerError(Some(error.to_string())))?;

        let user_entity = UserEntity {
            id: user_id.clone(),
            password_hash: None,
            email: None,
            user_type: UserType::GUEST,
            is_locked: false,
        };

        let user = match self.user_identity_repository
            .insert_user(&user_entity, None)
            .await {
            Ok(user) => {
                tracing::info!("Successfully inserted a new anonymous user data into database.");
                user
            },
            Err(db_err) => {
                tracing::error!("Failed to insert a new anonymous user data to database.");
                return Err(UserIdentityServiceError::DatabaseError(Some(format!("Failed to insert to database with error: {:?}", db_err.to_string()))))
            }
        };

        let event = AnonymousUserRegistered {
            user_type: user.user_type.clone(),
            id: user.id.clone(),
        };
        let user_publisher_clone = self.user_publisher.clone();

        tokio::spawn(async move {
            match user_publisher_clone
                .publish_anonymous_user_registered_event(event)
                .await {
                Ok(()) => {
                    tracing::info!("Successfully published the message.");
                },
                Err(err) => {
                    tracing::error!("Failed to publish the message with error: {:?}", err);
                }
            }
        }.instrument(Span::current()));

        let access_token = self.jwt_service.generate_token(
            &user.id.clone().to_string(),
            user.user_type.clone(),
            TokenType::ACCESS
        ).map_err(|error| return UserIdentityServiceError::ServerError(Some(error.to_string())))?;

        let res = UserRegisteredResponse {
            id: user.id.clone(),
            user_type: user.user_type.clone(),
            user_email: None,
        };

        Ok((access_token, refresh_token, res))
    }

    #[tracing::instrument(
        name = "Auth service - refresh access token function",
        skip(self, refresh_token, access_token)
    )]
    pub async fn refresh_access_token(
        &self,
        access_token: &str,
        refresh_token: &str
    ) -> Result<(String, String), UserIdentityServiceError> {
        //access token is already valid, no need for refreshing, return the same token
        let error = match self.jwt_service.validate_token(access_token) {
            Ok(_claims) => {
                tracing::info!("Access token is already valid. Not generating new one.");
                return Ok((access_token.to_string(), refresh_token.to_string()))
            },
            Err(error) => {
                error
            }
        };

        //refresh token if only it has expired
        let claims = match *error.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                //if token is expired get sub from expired token
                let claims = self.jwt_service.get_claims_from_token(access_token);
                match claims {
                    Err(e) => {
                        tracing::error!("Could not extract sub claim from jwt claims with error: {}.", e);
                        return Err(UserIdentityServiceError::TamperedJwtTokenError(None))
                    },
                    Ok(claims) => {
                        claims
                    }
                }
            },
            jsonwebtoken::errors::ErrorKind::InvalidSignature |
            jsonwebtoken::errors::ErrorKind::InvalidToken => {
                tracing::error!("Potentially tampered or malformed token detected.");
                return Err(UserIdentityServiceError::TamperedJwtTokenError(None));
            },
            _ => {
                tracing::error!("Internal JWT validation error: {:?}", error.kind());
                return Err(UserIdentityServiceError::TamperedJwtTokenError(None));
            }
        };

        let id = match Uuid::from_str(claims.sub.clone().as_str()) {
            Ok(id) => id,
            Err(_e) => {
                tracing::error!("Invalid id found in provided jwt.");
                return Err(UserIdentityServiceError::TamperedJwtTokenError(Some("Invalid id found in jwt.".to_string())))
            }
        };

        let refresh_tokens: Vec<RefreshTokenEntity> = match self.refresh_token_repository
            .get_refresh_token_by_user_id(id.clone())
            .await {
            Ok(tokens) => {
                tracing::info!("Successfully got tokens from database.");
                tokens
            },
            Err(db_err) => {
                tracing::error!("Failed to get tokens from database.");
                return Err(UserIdentityServiceError::DatabaseError(Some(format!("Failed to get tokens from database with error: {:?}", db_err.to_string()))))
            }
        };

        match self.jwt_service
            .validate_token(
                refresh_token
            ) {
            Ok(_claims) => {
                tracing::info!("Refresh token is valid.");
            },
            Err(_error) => {

                if claims.user_type == UserType::STANDARD {
                    tracing::error!("Refresh token has expired. Prompting standard user to log in again.");
                    return Err(UserIdentityServiceError::ExpiredRefreshTokenError(Some("Refresh token has expired. Log in again.".to_string())))
                }
                else {
                    tracing::info!("User is a guest user. Ignoring expired refresh token.");
                }

            }
        };

        for refresh_token_hash in refresh_tokens {
            if RefreshToken::verify_token(refresh_token, refresh_token_hash.hashed_value.0.expose_secret()) {
                if refresh_token_hash.is_revoked {
                    tracing::error!("Refresh token is revoked. Prompting user to log in again.");
                    return Err(UserIdentityServiceError::ExpiredRefreshTokenError(Some("Refresh token is revoked. Log in again.".to_string())))
                }
                break;
            };
        }

        let access_token = self.jwt_service
            .generate_token(
                &claims.sub.to_string(),
                claims.user_type.clone(),
                TokenType::ACCESS
            ).map_err(|error| {
                tracing::error!("Failed to generate jwt access token.");
                UserIdentityServiceError::ServerError(Some(error.to_string()))
        })?;

        let refresh_token = self.jwt_service.
            generate_token(
                &claims.sub.to_string(),
                claims.user_type.clone(),
                TokenType::REFRESH
            ).map_err(|error| {
            tracing::error!("Failed to generate jwt refresh token.");
            UserIdentityServiceError::ServerError(Some(error.to_string()))
        })?;

        Ok((access_token, refresh_token))
    }

    #[tracing::instrument(
        name = "Auth service - get user info function",
        skip(self)
    )]
    pub async fn get_user_info(
        &self,
        access_token: String
    ) -> Result<UserInfoResponse, UserIdentityServiceError> {
        let user_id = match self.jwt_service
            .validate_token(
                access_token.as_str()
            ) {
                Ok(claims) => {
                    tracing::info!("Access token is valid. Extracting user id.");
                    claims.sub
                },
                Err(error) => {
                    tracing::error!("Failed to validate token with error: {:?}", error.to_string());
                    return Err(UserIdentityServiceError::Unauthorized(Some("Access token invalid".to_string())));
                }
        };

        let user_id: Uuid = match Uuid::from_str(user_id.as_str()) {
            Ok(id) => id,
            Err(_e) => return Err(UserIdentityServiceError::TamperedJwtTokenError(Some("No valid user id found in token.".to_string()))),
        };

        let user = match self
            .user_identity_repository
            .get_user_by_id(&user_id)
            .await {
                Ok(user) => {
                    tracing::info!("Successfully fetched user by id.");
                    user
                },
                Err(e) => match e {
                    sqlx::Error::RowNotFound => {
                        return Err(UserIdentityServiceError::UserError(Some("User with that id does not exist.".to_string())))
                    }
                    _ => {
                        tracing::error!("Database error while fetching user by id: {:?}", e);
                        return Err(UserIdentityServiceError::DatabaseError(Some(e.to_string())))
                    }
                }
        };

        let res = UserInfoResponse {
            email: user.email,
            user_type: user.user_type,
            user_id: user.id,
        };

        Ok(res)
    }

    #[tracing::instrument(
        name = "Auth service - register standard user function",
        skip(self)
    )]
    pub async fn register_standard_user(
        &self,
        request: &RegisterStandardUserRequest,
        jwt: Option<String>
    ) -> Result<(String, String, UserRegisteredResponse), UserIdentityServiceError> {
        let mut registry_entity: UserEntity = match request.try_into() {
            Ok(r) => r,
            Err(e) => return Err(UserIdentityServiceError::RequestValidationError(Some(e.to_string()))),
        };

        let password =  match UserPassword::hash_password(registry_entity.password_hash.unwrap().as_ref()) {
            Ok(pw) => pw,
            Err(e) => return Err(UserIdentityServiceError::ServerError(Some(format!("Failed to compute hash for password: {}:", e.to_string()))))
        };



        registry_entity.password_hash = Some(UserPassword(SecretString::from(password)));

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

        let user = match self
            .user_identity_repository
            .insert_user(&mut registry_entity, old_id)
            .await {
            Ok(user) => {
                tracing::info!("User data successfully saved to database.");
                user
            },
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


        let user_clone = user.clone();
        let user_publisher_clone = self.user_publisher.clone();

        tokio::spawn(async move {
            let mut event: StandardUserRegistered = (&user_clone).into();

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
            id: user.id,
            user_type: user.user_type.clone(),
            user_email: user.email.clone(),
        };

        let access_token =  self.jwt_service.
            generate_token(
                &registry_entity.id.to_string(),
                registry_entity.user_type.clone(),
                TokenType::ACCESS
            ).map_err(|error| UserIdentityServiceError::ServerError(Some(error.to_string())))?;

        let refresh_token = self.jwt_service
            .generate_token(
                &registry_entity.id.to_string(),
                registry_entity.user_type.clone(),
                TokenType::REFRESH
            ).map_err(|error| UserIdentityServiceError::ServerError(Some(error.to_string())))?;

        Ok((access_token, refresh_token, res))
    }


    #[tracing::instrument(
        name = "Auth service - authenticate standard user function",
        skip(self)
    )]
    pub async fn authenticate_standard_user(
        &self,
        req: LoginStandardUserRequest
    ) -> Result<(String, String), UserIdentityServiceError> {
        let user = match self
            .user_identity_repository
            .get_user_by_email(&req.email)
            .await {
                Ok(user) => {
                    user
                },
                Err(error) => {
                    return match error {
                        sqlx::Error::RowNotFound => {
                            Err(UserIdentityServiceError::UserError(Some("User with that email does not exist".to_string())))
                        }
                        _ => Err(UserIdentityServiceError::DatabaseError(Some(format!("Error while retrieving user by email. {}", error.to_string()))))
                    };
            }
        };

        match UserPassword::verify_password(
            req.password.expose_secret(),
            user.password_hash.unwrap().as_ref()
        ) {
            Err(error) => {
                return Err(UserIdentityServiceError::ServerError(Some(format!("Error while verifying password. {}", error.to_string()))));
            },
            Ok(validated) => {
                if !validated {
                    return Err(UserIdentityServiceError::Unauthorized(Some("Invalid password.".to_string())));
                }
            }
        }

        let access_token = match self.jwt_service.generate_token(
            user.id.to_string().as_str(),
            UserType::STANDARD,
            TokenType::ACCESS
        ) {
            Err(error) => {
                return Err(UserIdentityServiceError::ServerError(Some(format!("Error while generating token. {}", error.to_string()))));
            },
            Ok(token) => {
                token
            }
        };

        let refresh_token = self.jwt_service.generate_token(
            &user.id.to_string(),
            user.user_type.clone(),
            TokenType::REFRESH
        )
            .map_err(|error| UserIdentityServiceError::ServerError(Some(error.to_string())))?;

        Ok((access_token, refresh_token))
    }

}
