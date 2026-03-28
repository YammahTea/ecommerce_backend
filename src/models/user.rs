use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    pub(crate) email: String,
    pub(crate) password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub(crate) id: Uuid,
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) hashed_password: String,
    pub(crate) created_at: DateTime<Utc>
}