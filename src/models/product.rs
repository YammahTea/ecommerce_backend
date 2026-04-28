use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::errors::product_error::{CreateProductError, ProductValidationError, UpdateProductError};
use crate::errors::product_error::ProductValidationError::{InvalidDescription, InvalidName, InvalidPrice, InvalidStockQuantity};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Product {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) price_in_cents: i32,
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
    pub(crate) price_in_cents: i32,
    pub(crate) stock_quantity: i32,
    pub(crate) status: ProductStatus
}

impl CreateProductRequest {
    pub fn validate(&self) -> Result<(), ProductValidationError> {
        if self.name.is_empty() {  return Err(InvalidName)  }
        if self.description.is_empty() {  return Err(InvalidDescription)  }
        if self.price_in_cents < 0 { return Err(InvalidPrice)  }
        if self.stock_quantity < 0 {  return Err(InvalidStockQuantity)  }
    
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct ProductPagination {
    pub(crate) page: Option<u32>,
    pub(crate) limit: Option<u32>
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateProductRequest {
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) price_in_cents: Option<i32>,
    pub(crate) stock_quantity: Option<i32>,
    pub(crate) status: Option<ProductStatus>
}

impl UpdateProductRequest {
    pub fn validate(&self) -> Result<(), ProductValidationError> {
        if let Some(name) = &self.name {
            if name.is_empty() { return Err(InvalidName) }
        }

        if let Some(description) = &self.description {
            if description.is_empty() { return Err(InvalidDescription) }
        }

        if let Some(price_in_cents) = self.price_in_cents {
            if price_in_cents < 0 { return Err(InvalidPrice) }
        }

         if let Some(stock_quantity) = self.stock_quantity {
            if stock_quantity < 0 { return Err(InvalidStockQuantity) }
        }

        Ok(())
    }
}