use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}
