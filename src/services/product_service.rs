use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::errors::product_error::{FetchProductError, ProductCreationError};
use crate::models::product::{CreateProductRequest, Product, ProductPagination};
use crate::repositories::product_repo::{fetch_product_by_id, fetch_products, insert_product};

// Note: "Admin" comment above a function name indicates
// that the service is used by an admin only endpoint

// Admin
pub async fn add_new_product(product_payload: &CreateProductRequest,  pool: &Pool<Postgres>) -> Result<Product, ProductCreationError> {

    if product_payload.name.is_empty() { return Err(ProductCreationError::InvalidName) }
    if product_payload.description.is_empty() { return Err(ProductCreationError::InvalidDescription) }
    if product_payload.price_in_cents < 0 { return Err(ProductCreationError::InvalidPrice) }
    if product_payload.stock_quantity < 0 { return Err(ProductCreationError::InvalidStockQuantity) }

    let created_product = insert_product(pool, product_payload).await?;
    Ok(created_product)

}

pub async fn list_products(product_pagination: ProductPagination, pool: &Pool<Postgres>) -> Result<Vec<Product>, FetchProductError> {

    let limit = product_pagination.limit.unwrap_or(10) as i32;

    // Edge case: if page = 0
    let raw_page = product_pagination.page.unwrap_or(1) as i32;
    let page = std::cmp::max(raw_page, 1);

    let safe_limit= std::cmp::min(limit, 30);
    let offset= (page - 1) * safe_limit ;

    let products: Vec<Product> = fetch_products(pool, offset, safe_limit).await?;
    Ok(products)
}


pub async fn get_single_product(product_id: Uuid, pool: &Pool<Postgres>) -> Result<Product, FetchProductError> {

    let product = fetch_product_by_id(pool, product_id).await?;

    match product {
        None => { Err(FetchProductError::ProductNotFound) }
        Some(fetched_product) => { Ok(fetched_product) }
    }
}