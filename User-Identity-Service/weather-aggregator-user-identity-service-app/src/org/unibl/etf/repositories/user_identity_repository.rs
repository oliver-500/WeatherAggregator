
use secrecy::SecretString;
use sqlx::PgPool;
use secrecy::ExposeSecret;
use crate::org::unibl::etf::model::domain::entities::user_entity::user_email::UserEmail;
use crate::org::unibl::etf::model::domain::entities::user_entity::user_password::UserPassword;
use crate::org::unibl::etf::model::domain::entities::user_entity::UserEntity;
use crate::org::unibl::etf::model::user_type::UserType;
use uuid::Uuid;

#[derive(Debug)]
pub struct UserIdentityRepository{
    db_pool: PgPool
}

impl UserIdentityRepository {
    pub fn new_with_db_pool(pool: PgPool) -> Self {
        Self {
            db_pool: pool
        }
    }

    #[tracing::instrument(
        name = "Saving new user into database method",
        skip(self)
    )]
    pub async fn insert_user<'a>(
        &self,
        user_entity: &'a UserEntity,
        old_id: Option<Uuid>
    ) -> Result<&'a UserEntity, sqlx::Error> {
        let mut tx = self.db_pool.begin().await?;

        sqlx::query!(
            r#"
            INSERT INTO wa_user (id, email, password_hash, user_type, is_locked)
            VALUES ($1, $2, $3, $4::user_account_type, $5)
            "#,
            user_entity.id,
            user_entity.email.as_ref().map(|e| e.0.clone()),
            user_entity.password_hash.as_ref().map(|p| p.0.expose_secret()),
            user_entity.user_type.clone() as UserType,
            user_entity.is_locked
        )
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                e
            })?;

        if let Some(old_id) = old_id {
            sqlx::query!(
                r#"
                DELETE FROM wa_user
                WHERE id = $1
                "#,
                old_id
            ).execute(&mut *tx)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to delete user {}: {:?}", old_id, e);
                    e
                })?;
        }

        tx.commit().await?;

        Ok(user_entity)
    }

    #[tracing::instrument(
        name = "Deleting user from the database",
        skip(self)
    )]
    pub async fn delete_user_by_id(&self, id: &Uuid) -> Result<(), sqlx::Error> {
        let result = sqlx::query!(
        r#"
        DELETE FROM wa_user
        WHERE id = $1
        "#,
        id
    )
            .execute(&self.db_pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to delete user {}: {:?}", id, e);
                e
            })?;

        if result.rows_affected() == 0 {
            tracing::warn!("Delete attempted for user {}, but no record was found.", id);
            // Depending on your preference, you could return an Error here
            // if you expect the user to always exist.
        }

        Ok(())
    }


    #[tracing::instrument(
        name = "Get user by email from database method",
        skip(self)
    )]
    pub async fn get_user_by_email(&self, email: &str) -> Result<UserEntity, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT id, email, password_hash, user_type as "user_type: UserType", is_locked
            FROM wa_user
            WHERE email = $1
            "#,
            email
        )
            .fetch_one(&self.db_pool)
            .await?;

        // 2. Map raw strings to your Domain Types
        // Assuming UserEmail and Password have a .new() or parse method
        Ok(UserEntity {
            id: row.id,
            // .map wraps the inner String into your NewType only if it exists
            email: row.email.map(UserEmail),
            password_hash: row.password_hash.map(|h| UserPassword(SecretString::from(h))),
            user_type: row.user_type,
            is_locked: row.is_locked,
        })
    }


    #[tracing::instrument(
        name = "Saving new standard user in the database method",
        skip(self)
    )]
    pub async fn get_user_by_id(&self, id: &Uuid) -> Result<UserEntity, sqlx::Error> {
        let row = sqlx::query!(
        r#"
        SELECT id, email, password_hash, user_type as "user_type: UserType", is_locked
        FROM wa_user
        WHERE id = $1
        "#,
        id
    )
            .fetch_one(&self.db_pool)
            .await?;

        Ok(UserEntity {
            id: row.id,
            // .map wraps the inner String into your NewType only if it exists
            email: row.email.map(UserEmail),
            password_hash: row.password_hash.map(|h| UserPassword(SecretString::from(h))),
            user_type: row.user_type,
            is_locked: row.is_locked,

        })
    }

}
