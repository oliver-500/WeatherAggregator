use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct UserRegisteredResponse {
    pub id: Uuid
}