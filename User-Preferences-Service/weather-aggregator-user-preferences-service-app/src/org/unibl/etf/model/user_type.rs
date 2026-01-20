use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "user_account_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserType {
    GUEST,
    STANDARD
}