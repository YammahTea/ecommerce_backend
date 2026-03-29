use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use email_address::EmailAddress;

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    pub(crate) email: EmailAddress,
    pub(crate) password: String
}

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub(crate) identifier: String,
    pub(crate) password: String
}

#[derive(Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    pub(crate) id: Uuid,
    pub(crate) email: String,
    pub(crate) username: Option<String>,
    pub(crate) hashed_password: String,
    pub(crate) created_at: DateTime<Utc>
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub(crate) id: Uuid,
    pub(crate) email: String,
    pub(crate) username: Option<String>,
    pub(crate) created_at: DateTime<Utc>
}