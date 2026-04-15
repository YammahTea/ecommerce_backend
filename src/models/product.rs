use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Product {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) price: i32,
    pub(crate) stock_quantity: i32,
    pub(crate) status: ProductStatus,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
    pub(crate) deleted_at: Option<DateTime<Utc>>
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone)]
#[serde(rename_all = "lowercase")]  // JSON boundary: client must send "draft"/"active"/"archived" exactly
#[sqlx(type_name = "VARCHAR")]
#[sqlx(rename_all = "lowercase")]   // DB boundary: stored as "draft"/"active"/"archived" to match CHECK constraint
pub enum ProductStatus {
    Draft,
    Active,
    Archived,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateProductRequest {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) price: i32,
    pub(crate) stock_quantity: i32,
    pub(crate) status: ProductStatus
}