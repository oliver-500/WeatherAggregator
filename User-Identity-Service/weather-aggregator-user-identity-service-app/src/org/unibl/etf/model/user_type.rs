use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "user_account_type")] // Must match the name in CREATE TYPE exactly
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")] // Matches 'GUEST' and 'STANDARD'
pub enum UserType {
    GUEST,
    STANDARD
}