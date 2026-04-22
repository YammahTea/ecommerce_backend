use sqlx::{Pool, Postgres};
use crate::errors::product_error::ProductCreationError;
use crate::models::product::{CreateProductRequest, Product};
use crate::repositories::product_repo::insert_product;

pub async  fn add_new_product(product_payload: &CreateProductRequest,  pool: &Pool<Postgres>)
    -> Result<Product, ProductCreationError> {

    if product_payload.name.is_empty() { return Err(ProductCreationError::InvalidName) }
    if product_payload.description.is_empty() { return Err(ProductCreationError::InvalidDescription) }
    if product_payload.price_in_cents < 0 { return Err(ProductCreationError::InvalidPrice) }
    if product_payload.stock_quantity < 0 { return Err(ProductCreationError::InvalidStockQuantity) }

    let created_product = insert_product(pool, product_payload).await?;
    Ok(created_product)

}