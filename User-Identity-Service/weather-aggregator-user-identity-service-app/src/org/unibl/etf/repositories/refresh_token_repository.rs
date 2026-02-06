
use secrecy::SecretString;
use secrecy::ExposeSecret;
use sqlx::PgPool;
use crate::org::unibl::etf::model::domain::entities::user_entity::refresh_token::RefreshToken;

use uuid::Uuid;
use crate::org::unibl::etf::model::domain::entities::refresh_token_entity::RefreshTokenEntity;

#[derive(Debug)]
pub struct RefreshTokenRepository{
    db_pool: PgPool
}

impl RefreshTokenRepository {
    pub fn new_with_db_pool(pool: PgPool) -> Self {
        Self {
            db_pool: pool
        }
    }

    #[tracing::instrument(
        name = "Saving access token into database method",
        skip(self)
    )]
    pub async fn insert_refresh_token(
        &self,
        id: Uuid,
        user_id: Uuid,
        token: &RefreshToken,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO refresh_token (id, user_id, hashed_value, is_revoked)
            VALUES ($1, $2, $3, $4)
            "#,
            id,
            user_id,
            token.0.expose_secret(),
            false
        ).execute(&self.db_pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query with error: {:?}", e);
                e
            })?;

       Ok(())
    }

    #[tracing::instrument(
        name = "Get refresh token from database by user id method.",
        skip(self)
    )]
    pub async fn get_refresh_token_by_user_id(
        &self,
        user_id: Uuid
    ) -> Result<Vec<RefreshTokenEntity>, sqlx::Error> {
        let rows = sqlx::query!(
        r#"
        SELECT id, user_id, hashed_value, is_revoked
        FROM refresh_token
        WHERE user_id = $1
        "#,
        user_id
        ).fetch_all(&self.db_pool)
            .await?;

        let tokens = rows
            .into_iter()
            .map(|row| RefreshTokenEntity {
                id: row.id,
                user_id: row.user_id,
                hashed_value: RefreshToken(SecretString::from(row.hashed_value)),
                is_revoked: row.is_revoked,
            })
            .collect();

        Ok(tokens)
    }

}
