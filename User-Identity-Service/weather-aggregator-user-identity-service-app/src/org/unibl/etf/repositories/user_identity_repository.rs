use sqlx::PgPool;

use crate::org::unibl::etf::model::domain::entities::user_entity::UserEntity;

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




}

// impl Default for UserIdentityRepository {
//     fn default() -> Self {
//         Self::new()
//     }
// }