use std::str::FromStr;
use secrecy::{ExposeSecret, SecretString};
use tracing::{Instrument, Span};
use uuid::Uuid;
use crate::org::unibl::etf::jwt::token_type::TokenType;
use crate::org::unibl::etf::model::domain::entities::user_entity::refresh_token::RefreshToken;
use crate::org::unibl::etf::model::domain::entities::user_entity::user_email::UserEmail;
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

    #[tracing::instrument(name = "Auth service - get user info function", skip(
        self
    ))]
    pub async fn get_user_info(
        &self,
        access_token: String
    ) -> Result<UserInfoResponse, UserIdentityServiceError> {
        let user_id = match self.jwt_service.validate_token(access_token.as_str()) {
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
            Err(e) => return Err(UserIdentityServiceError::ServerError(Some(e.to_string()))),
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
                // Specifically catch the "Not Found" case
                sqlx::Error::RowNotFound => {
                    return Err(UserIdentityServiceError::UserError(Some("User with that id does not exist.".to_string())))
                }
                // Catch other DB issues (like unique violations or connection loss)
                _ => {
                    tracing::error!("Database error: {:?}", e);
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

    #[tracing::instrument(name = "Auth service - authenticate standard user function", skip(
        self
    ))]
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
                match error {
                    sqlx::Error::RowNotFound => {
                        return Err(UserIdentityServiceError::UserError(Some("User with that email does not exist".to_string())))
                    }
                    _ => return Err(UserIdentityServiceError::DatabaseError(Some(format!("Error while retrieving user by email. {}", error.to_string()))))
                }

            }
        };

        match UserPassword::verify_password(req.password.expose_secret(), user.password_hash.unwrap().as_ref()) {
            Err(error) => {
                return Err(UserIdentityServiceError::ServerError(Some(format!("Error while verifying password. {}", error.to_string()))));
            },
            Ok(validated) => {
                if !validated {
                    return Err(UserIdentityServiceError::Unauthorized(Some(format!("Invalid password."))));
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


        let refresh_token = match self.jwt_service.generate_token(&user.id.to_string(), user.user_type.clone(), TokenType::REFRESH) {
            Ok(token) => {
                token
            },
            Err(error) => {
                return Err(UserIdentityServiceError::ServerError(Some(error.to_string())));
            }
        };

        Ok((access_token, refresh_token))
    }

    #[tracing::instrument(name = "Auth service - register standard user function", skip(
        self
    ))]
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

        let refresh_token = match self.jwt_service.generate_token(&registry_entity.id.to_string(), registry_entity.user_type.clone(), TokenType::REFRESH) {
            Ok(token) => {
                token
            },
            Err(error) => {
                return Err(UserIdentityServiceError::ServerError(Some(error.to_string())));
            }
        };

        registry_entity.password_hash = Some(UserPassword(SecretString::from(password)));
        registry_entity.refresh_token_hash = Some(RefreshToken(SecretString::from(RefreshToken::hash_refresh_token(refresh_token.as_str()))));

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







        let user_clone = user.clone();
        let user_publisher_clone = self.user_publisher.clone();

        tokio::spawn( async move {
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

        let access_token = match self.jwt_service.generate_token(&user.id.to_string(), user.user_type.clone(), TokenType::ACCESS) {
            Ok(token) => {
                token
            },
            Err(error) => {
                return Err(UserIdentityServiceError::ServerError(Some(error.to_string())));
            }
        };

        Ok((access_token, refresh_token, res))
    }

    #[tracing::instrument(name = "Auth service - register anonymous user function",
        skip(self))
    ]
    pub async fn register_anonymous_user(&self) -> Result<(String, String, UserRegisteredResponse), UserIdentityServiceError> {
        let user_id = Uuid::new_v4();
        let refresh_token = match self.jwt_service.generate_token(user_id.clone().to_string().as_str(), UserType::GUEST, TokenType::REFRESH) {
            Ok(token) => {
                token
            },
            Err(error) => {
                return Err(UserIdentityServiceError::ServerError(Some(error.to_string())));
            }
        };

        let user_entity = UserEntity {
            id: user_id.clone(),
            password_hash: None,
            email: None,
            user_type: UserType::GUEST,
            is_locked: false,
            refresh_token_hash: Some(RefreshToken(SecretString::from(RefreshToken::hash_refresh_token(refresh_token.as_str())))),
        };

        match self.user_identity_repository
            .insert_user(&user_entity, None)
            .await {
            Ok(_r) => {
                tracing::info!("Successfully inserted user to database.");
            },
            Err(db_err) => {
                tracing::error!("Failed to insert a user to database.");
                return Err(UserIdentityServiceError::DatabaseError(Some(format!("{:?}", db_err.to_string()))))
            }
        }

        let user = AnonymousUserRegistered {
            user_type: UserType::GUEST,
            id: user_id.clone(),
        };
        let user_publisher_clone = self.user_publisher.clone();

        tokio::spawn( async move {
            match user_publisher_clone
                .publish_anonymous_user_registered_event(user)
                .await {
                    Ok(_) => {
                        tracing::info!("Successfully published a message");
                    },
                    Err(e) => {
                        tracing::error!("Failed to publish message: Error: {:?}", e);
                    }
            }
        }.instrument(Span::current()));

        match self.jwt_service.generate_token(&user_id.to_string(), UserType::GUEST, TokenType::ACCESS) {
            Ok(access_token) => {
                let res = UserRegisteredResponse {
                    id: user_id,
                    user_type: UserType::GUEST,
                    user_email: None,
                };
                Ok((access_token, refresh_token, res))
            },
            Err(error) => {
                Err(UserIdentityServiceError::ServerError(Some(error.to_string())))
            }
        }
    }

    #[tracing::instrument(name = "Auth service - refresh access token function", skip(
        self
    ))]
    pub async fn refresh_access_token(&self, access_token: &str, refresh_token: &str) -> Result<(String, String), UserIdentityServiceError> {
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
        let sub = match *error.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                //if token is expired get sub from expired token
                let claims = self.jwt_service.get_claims_from_token(access_token);

                if let Err(e) = claims {
                    tracing::error!("Could not extract sub from jwt claims: {}.", e);
                    return Err(UserIdentityServiceError::TamperedJwtTokenError(None))
                }

                let claims = claims.unwrap();
                claims.sub.to_string()
            },
            jsonwebtoken::errors::ErrorKind::InvalidSignature |
            jsonwebtoken::errors::ErrorKind::InvalidToken => {
                tracing::warn!("Potentially tampered or malformed token detected.");
                return Err(UserIdentityServiceError::TamperedJwtTokenError(None));
            },
            _ => {
                tracing::error!("Internal JWT validation error: {:?}", error.kind());
                return Err(UserIdentityServiceError::ServerError(None));
            }
        };

        let id = Uuid::from_str(sub.clone().as_str());

        let id = match id {
            Ok(id) => id,
            Err(_e) => {
                tracing::error!("Invalid id found in provided jwt.");
                return Err(UserIdentityServiceError::TamperedJwtTokenError(Some(format!("Invalid id found in jwt."))))
            }
        };

        let user = match self.user_identity_repository
            .get_user_by_id(&id)
            .await {
                Ok(user) => {
                    tracing::info!("Successfully fetched user by id.");
                    user
                },
                Err(db_err) => {
                    tracing::error!("Failed to fetch user by id.");
                    return Err(UserIdentityServiceError::DatabaseError(Some(format!("{:?}", db_err.to_string()))))
                }
        };

        if let None = user.refresh_token_hash {
            tracing::error!("Refresh token not found in database");
            return Err(UserIdentityServiceError::ServerError(Some("Refresh token not found in db.".to_string())))
        }
        let refresh_token_hash = user.refresh_token_hash.unwrap();

        if !RefreshToken::verify_token(refresh_token, refresh_token_hash.0.expose_secret()) {
            tracing::error!("Refresh token stored in database does not match the one provided in request.");
            return Err(UserIdentityServiceError::TamperedJwtTokenError(Some("Refresh token invalid.".to_string())))
        };

        let access_token = match self.jwt_service.generate_token(&user.id.to_string(), user.user_type.clone(), TokenType::ACCESS) {
            Ok(token) => {
               token
            },
            Err(error) => {
                tracing::error!("Failed to generate jwt token.");
                return Err(UserIdentityServiceError::ServerError(Some(error.to_string())));
            }
        };

        let refresh_token = match self.jwt_service.generate_token(&user.id.to_string(), user.user_type.clone(), TokenType::REFRESH) {
            Ok(token) => {
                token
            },
            Err(error) => {
                tracing::error!("Failed to generate jwt token.");
                return Err(UserIdentityServiceError::ServerError(Some(error.to_string())));
            }
        };

        Ok((access_token, refresh_token))
    }

}


// impl Default for AuthService {
//     fn default() -> Self {
//         Self::new()
//     }
// }
