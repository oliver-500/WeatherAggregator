use std::ops::Deref;
use uuid::Uuid;
use crate::org::unibl::etf::model::domain::entities::user_entity::UserEntity;
use crate::org::unibl::etf::model::domain::messages::anonymous_user_registered::AnonymousUserRegistered;
use crate::org::unibl::etf::model::errors::user_identity_service_error::UserIdentityServiceError;
use crate::org::unibl::etf::model::requests::register_standard_user_request::RegisterStandardUserRequest;
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

    #[tracing::instrument(name = "Auth service - register standard user function", skip(
        self
    ))]
    pub async fn register_standard_user(
        &self,
        request: &RegisterStandardUserRequest,
        jwt: Option<String>
    ) -> Result<UserEntity, UserIdentityServiceError> {

        //sacuvati u lokalnu bazu pw i username
        let mut registry_entity: UserEntity = match request.try_into() {
            Ok(r) => r,
            Err(e) => return Err(UserIdentityServiceError::RequestValidationError(Some(e.to_string()))),
        };

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

        // i poslati username(email) email profile servisu putem eventa userRegisteredEvent

        let event: AnonymousUserRegistered = (res).into();
        match self
            .user_publisher
            .publish_user_registered_event(event)
            .await
            .map_err(|e| {
                UserIdentityServiceError::ServerError(Some(e.to_string()))
            }) {
            Ok(r) => r,
            Err(e) => {
                return Err(e);
            }
        };

        //a poslati i user preferences servisu userRegistered(generiusati id) ili guestUserRegistered(old id new id generisati new id) u zavisnosti da li je jwt prisutan
        return Ok(res.deref().clone());
    }

    #[tracing::instrument(name = "Auth service - register anonymous user function", skip(
        self
    ))]
    pub async fn register_anonymous_user(&self) -> Result<String, UserIdentityServiceError> {
        let user_id = Uuid::new_v4();

        match self.jwt_service.generate_token(&user_id.to_string(), UserType::GUEST) {
            Ok(token) => {
                Ok(token)
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
