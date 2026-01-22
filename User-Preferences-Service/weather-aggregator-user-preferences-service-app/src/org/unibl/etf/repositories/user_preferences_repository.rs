use sqlx::PgPool;
use uuid::Uuid;
use crate::org::unibl::etf::model::domain::entities::user_preferences_entity::UserPreferencesEntity;
use crate::org::unibl::etf::model::user_type::UserType;
use crate::org::unibl::etf::model::domain::entities::user_preferences_entity::unit_system_type::UnitSystemType;
use crate::org::unibl::etf::model::domain::entities::user_preferences_with_history::UserPreferencesWithHistory;
use crate::org::unibl::etf::model::domain::entities::location_history_entity::LocationHistoryEntity;
#[derive(Debug)]
pub struct UserPreferencesRepository {
    pub db_pool: PgPool
}

impl UserPreferencesRepository {
    pub fn new_with_db_pool(pool: PgPool) -> Self {
        Self {
            db_pool: pool
        }
    }

    #[tracing::instrument(name = "Saving user preferences in the database", skip(self, user_preferences_entity))]
    pub async fn save(
        &self,
        user_preferences_entity: &UserPreferencesEntity,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
        r#"
        INSERT INTO user_preferences
            (user_id, user_type, unit_system, favorite_location_name, favorite_lat, favorite_lon)
        VALUES ($1, COALESCE($2, 'GUEST'::user_account_type), $3, $4, $5::FLOAT8, $6::FLOAT8)
        ON CONFLICT (user_id) DO UPDATE SET
            user_type = CASE WHEN $2 IS NULL THEN user_preferences.user_type ELSE EXCLUDED.user_type END,
            unit_system = EXCLUDED.unit_system,
            favorite_location_name = EXCLUDED.favorite_location_name,
            favorite_lat = EXCLUDED.favorite_lat,
            favorite_lon = EXCLUDED.favorite_lon,
            updated_at = NOW()
        "#,
        user_preferences_entity.user_id,
        user_preferences_entity.user_type as _, // The "as _" helps with custom types
        user_preferences_entity.unit_system as _,
        user_preferences_entity.favorite_location_name, // Option<String>
        user_preferences_entity.favorite_lat,          // Option<f64>
        user_preferences_entity.favorite_lon,          // Option<f64>
    )
            .execute(&self.db_pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                e
            })?;

        Ok(())
    }

    #[tracing::instrument(name = "Saving user preferences in the database", skip(self))]
    pub async fn find_by_id_with_history(&self, id: Uuid) -> Result<UserPreferencesWithHistory, sqlx::Error> {
        let prefs = sqlx::query_as!(
        UserPreferencesEntity,
        r#"
        SELECT
            user_id,
            user_type as "user_type: UserType",
            unit_system as "unit_system: UnitSystemType",
            favorite_location_name,
            favorite_lat::FLOAT8 as "favorite_lat?",
            favorite_lon::FLOAT8 as "favorite_lon?",
            updated_at as "updated_at!"
        FROM user_preferences
        WHERE user_id = $1
        "#,
        id
    )
            .fetch_one(&self.db_pool)
            .await?;


        let history = sqlx::query_as!(
        LocationHistoryEntity,
        r#"
        SELECT
            id,
            user_id,
            location_name,
            lat::FLOAT8 as "lat!",
            lon::FLOAT8 as "lon!",
            searched_at as "searched_at!"
        FROM location_history
        WHERE user_id = $1
        ORDER BY searched_at DESC
        "#,
        id
    )
            .fetch_all(&self.db_pool)
            .await?;

        Ok(UserPreferencesWithHistory {
            preferences: prefs,
            history,
        })

    }

    #[tracing::instrument(name = "Adding history item and trimming to N", skip(self))]
    pub async fn add_history_item(
        &self,
        item: LocationHistoryEntity,
        limit: i64, // Pass N here (e.g., 10)
    ) -> Result<Uuid, sqlx::Error> {
        let mut tx = self.db_pool.begin().await?;

        // 1. Insert the new location
        let row = sqlx::query!(
        r#"
        INSERT INTO location_history (user_id, location_name, lat, lon)
        VALUES ($1, $2, $3::FLOAT8, $4::FLOAT8)
        RETURNING id
        "#,
        item.user_id,
        item.location_name,
        item.lat,
        item.lon
    )
            .fetch_one(&mut *tx)
            .await?;

        let new_id = row.id;

        // 2. Delete oldest items exceeding the limit N
        // This subquery finds the IDs of the newest N items and deletes everything else
        sqlx::query!(
        r#"
        DELETE FROM location_history
        WHERE id IN (
            SELECT id FROM location_history
            WHERE user_id = $1
            ORDER BY searched_at DESC
            OFFSET $2
        )
        "#,
        item.user_id,
        limit
    )
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(new_id)
    }

    #[tracing::instrument(name = "Migrating guest data to registered user", skip(self))]
    pub async fn migrate_guest_to_user(
        &self,
        guest_id: Uuid,
        registered_user_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.db_pool.begin().await?;

        // 1. Update the main preferences record
        //ako ne postoji guest id, ne mora fail da se desi
        sqlx::query!(
        r#"
        UPDATE user_preferences
        SET user_id = $1, user_type = $2, updated_at = NOW()
        WHERE user_id = $3
        "#,
        registered_user_id,
        UserType::STANDARD as _, // Type hint goes here, in the argument list
        guest_id
    )
            .execute(&mut *tx)
            .await?;

        // match result {
        //     Ok(res) => {
        //         let rows_affected = res.rows_affected();
        //         if rows_affected == 0 {
        //             tracing::warn!("No guest record found to update. Continuing anyway.");
        //         }
        //     }
        //     Err(e) => {
        //         // Log the error but don't use '?' so the function continues
        //         return Err(e);
        //     }
        // }

        // 2. Update the history records
        sqlx::query!(
        r#"
        UPDATE location_history
        SET user_id = $1
        WHERE user_id = $2
        "#,
        registered_user_id,
        guest_id
    )
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }


}