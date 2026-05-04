use sqlx::{Pool, Postgres};
use tracing::instrument;
use uuid::Uuid;
use crate::errors::product_error::{FetchProductError, CreateProductError, UpdateProductError, SoftDeleteProductError};
use crate::models::product::{CreateProductRequest, Product, ProductPagination, UpdateProductRequest};
use crate::repositories::product_repo::{fetch_product_by_id, fetch_products, insert_product, soft_delete_product, update_product};

// Note: "Admin" comment above a function name indicates
// that the service is used by an admin only endpoint

// Admin
#[instrument(skip(pool, product_payload), fields(
    name = %product_payload.name,
    price = %product_payload.price_in_cents,
    stock_quantity = %product_payload.stock_quantity,
    status = ?product_payload.status
))]
pub async fn add_new_product(product_payload: &CreateProductRequest,  pool: &Pool<Postgres>) -> Result<Product, CreateProductError> {

    product_payload.validate()?;

    let created_product = insert_product(pool, product_payload).await?;
    Ok(created_product)

}

#[instrument(skip(pool, product_pagination), fields(
    limit = tracing::field::Empty,
    page = tracing::field::Empty
))]
pub async fn list_products(product_pagination: ProductPagination, pool: &Pool<Postgres>) -> Result<Vec<Product>, FetchProductError> {

    let limit = product_pagination.limit.unwrap_or(10) as i32;

    // Edge case: if page = 0
    let raw_page = product_pagination.page.unwrap_or(1) as i32;
    let page = std::cmp::max(raw_page, 1);

    let safe_limit= std::cmp::min(limit, 30);
    let offset= (page - 1) * safe_limit;

    // to log the sanitized values
    tracing::Span::current().record("limit", safe_limit);
    tracing::Span::current().record("page", page);

    let products: Vec<Product> = fetch_products(pool, offset, safe_limit).await?;
    Ok(products)
}

#[instrument(skip(pool), fields(product_id = %product_id))]
pub async fn get_single_product(product_id: Uuid, pool: &Pool<Postgres>) -> Result<Product, FetchProductError> {

    let product = fetch_product_by_id(pool, product_id).await?;

    match product {
        None => { Err(FetchProductError::ProductNotFound) }
        Some(fetched_product) => { Ok(fetched_product) }
    }
}

// Admin
#[instrument(skip(pool, product_info), fields(product_id = %product_id))]
pub async fn edit_product(product_id: Uuid, product_info: &UpdateProductRequest, pool: &Pool<Postgres>) -> Result<Product, UpdateProductError> {

    product_info.validate()?;

    let product = update_product(pool, product_id, product_info).await?;

    match product {
        None => { Err(UpdateProductError::ProductNotFound) }
        Some(updated_product) => { Ok(updated_product) }
    }

}

// Admin
#[instrument(skip(pool), fields(product_id = %product_id))]
pub async fn remove_product(product_id: Uuid, pool: &Pool<Postgres>) -> Result<String, SoftDeleteProductError> {

    let product_to_delete = soft_delete_product(pool, product_id).await;
    product_to_delete
}

