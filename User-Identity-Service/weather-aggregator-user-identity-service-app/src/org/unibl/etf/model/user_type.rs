use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum UserType {
    GUEST,
    STANDARD
}