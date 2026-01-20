use secrecy::SecretString;
use sqlx::PgPool;
use crate::org::unibl::etf::model::domain::entities::user_entity::user_email::UserEmail;
use crate::org::unibl::etf::model::domain::entities::user_entity::user_password::UserPassword;
use crate::org::unibl::etf::model::domain::entities::user_entity::UserEntity;
use crate::org::unibl::etf::model::user_type::UserType;

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

    #[tracing::instrument(name = "Saving new standard user in the database method", skip())]
    pub async fn insert_user<'a>(
        &self,
        user_entity: &'a UserEntity,
    ) -> Result<&'a UserEntity, sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO wa_user (id, email, password_hash)
            VALUES ($1, $2, $3)
            "#,
            user_entity.id,
            user_entity.email.as_ref(),
            user_entity.password.as_ref(),
        )
            .execute(&self.db_pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                e
            })?;
        Ok(user_entity)
    }


    #[tracing::instrument(name = "Saving new standard user in the database method", skip())]
    pub async fn get_user_by_email(&self, email: &str) -> Result<UserEntity, sqlx::Error> {
        let row = sqlx::query!(
        r#"
        SELECT id, email, password_hash
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
            email: UserEmail(row.email), // Or however you initialize your Value Object
            password: UserPassword(SecretString::from(row.password_hash)),
            user_type: UserType::STANDARD // Mapping 'password_hash' column to 'password' field
        })
    }
}

// impl Default for UserIdentityRepository {
//     fn default() -> Self {
//         Self::new()
//     }
// }