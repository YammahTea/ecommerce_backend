use sqlx::{Pool, Postgres};
use tracing::{error, instrument};
use uuid::Uuid;
use crate::errors::product_error::{FetchProductError, CreateProductError, UpdateProductError, SoftDeleteProductError};
use crate::models::product::{CreateProductRequest, Product, UpdateProductRequest};

// NOTE: using instrument, please skip everything, fields are logged in the "services" layer ONLY

#[instrument(skip(pool, input))]
pub async fn insert_product(pool: &Pool<Postgres>, input: &CreateProductRequest) -> Result<Product, CreateProductError> {
    let query = r#"
        INSERT INTO products
        (name, description, price_in_cents, stock_quantity, status)
        VALUES
        ($1, $2, $3, $4, $5)
        RETURNING *
        "#;

    sqlx::query_as::<_, Product>(query)
        .bind(&input.name)
        .bind(&input.description)
        .bind(&input.price_in_cents)
        .bind(&input.stock_quantity)
        .bind(&input.status)
        .fetch_one(pool).await
        .map_err(|error_message| {
            error!(error = ?error_message, "Error occurred while creating a product");
            CreateProductError::DatabaseError
        })


}

#[instrument(skip(pool, offset, limit))]
pub async fn fetch_products(pool: &Pool<Postgres>, offset: i32, limit: i32) -> Result<Vec<Product>, FetchProductError> {

    let query = r#"
        SELECT * FROM products
        WHERE status = 'active' AND deleted_at IS NULL
        ORDER BY created_at DESC
        LIMIT $1
        OFFSET $2
        "#;

    sqlx::query_as::<_, Product>(query)
        .bind(&limit)
        .bind(&offset)
        .fetch_all(pool).await
        .map_err(|error_message| {
            error!(error = ?error_message, limit = %limit, offset = %offset, "Error occurred while fetching products");
            FetchProductError::DatabaseError
        })

}

#[instrument(skip(pool, product_id))]
pub async fn fetch_product_by_id(pool: &Pool<Postgres>, product_id: Uuid) -> Result<Option<Product>, FetchProductError> {

    let query = r#"
        SELECT * FROM products
        WHERE id = $1
        AND status = 'active' AND deleted_at IS NULL
        "#;

    sqlx::query_as::<_, Product>(query)
        .bind(&product_id)
        .fetch_optional(pool).await
        .map_err(|error_message| {
            error!(error = ?error_message, product_id = %product_id, "Error occurred while fetching product");
            FetchProductError::DatabaseError
        })
}

#[instrument(skip(pool, product_id, input))]
pub async fn update_product(pool: &Pool<Postgres>, product_id:Uuid, input: &UpdateProductRequest) -> Result<Option<Product>, UpdateProductError> {
    let query = r#"
        UPDATE products
        SET
        name = COALESCE($1, name),
        description = COALESCE($2, description),
        price_in_cents = COALESCE($3, price_in_cents),
        stock_quantity = COALESCE($4, stock_quantity),
        status = COALESCE($5, status),
        updated_at = NOW()
        WHERE id = $6
        RETURNING *
    "#;

    sqlx::query_as::<_, Product>(query)
        .bind(&input.name)
        .bind(&input.description)
        .bind(&input.price_in_cents)
        .bind(&input.stock_quantity)
        .bind(&input.status)
        .bind(&product_id)
        .fetch_optional(pool).await
        .map_err(|error_message| {
            error!(error = ?error_message, "Error occurred while updating product");
            UpdateProductError::DatabaseError
        })

}

#[instrument(skip(pool, product_id))]
pub async fn soft_delete_product(pool: &Pool<Postgres>, product_id:Uuid) -> Result<String, SoftDeleteProductError> {

    let query = r#"
        UPDATE products
        SET
        deleted_at = NOW(),
        status = 'archived'
        WHERE id = $1
    "#;

    let result = sqlx::query(query)
        .bind(&product_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(SoftDeleteProductError::ProductNotFound)
    }

    Ok("Product deleted successfully.".to_string())
}
