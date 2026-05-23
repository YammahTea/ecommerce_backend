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
    pub(crate) role: String,
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

#[derive(Serialize, Debug)]
pub struct UserLoginResponse {
    pub(crate) access_token: String,
    pub(crate) raw_token: String
}

#[derive(Deserialize, Debug)]
pub struct RefreshTokenRequest {
    pub(crate) raw_refresh_token: String
}

#[derive(Debug, sqlx::FromRow)]
pub struct RefreshTokenInfo {
    pub(crate) token_id: Uuid,
    pub(crate) user_id: Uuid,
    pub(crate) role: String // matches the role column returned from the users table
}