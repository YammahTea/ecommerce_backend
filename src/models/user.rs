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
pub struct User {
    pub(crate) id: Uuid,
    // TODO: Add username column to database
    // pub(crate) username: Option<String>,
    pub(crate) email: String,
    pub(crate) hashed_password: String,
    pub(crate) created_at: DateTime<Utc>
}