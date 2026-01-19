use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Debug, Deserialize)]
pub struct AnonymousUserRegistered {
    pub email: String,
    pub id: Uuid,
}
