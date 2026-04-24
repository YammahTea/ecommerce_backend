use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::errors::product_error::{FetchProductError, ProductCreationError};
use crate::models::product::{CreateProductRequest, Product};

pub async fn insert_product(pool: &Pool<Postgres>, input: &CreateProductRequest) -> Result<Product, ProductCreationError> {
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
        .map_err(|e| {
            eprintln!("Error occurred while creating a product in repositories/product_repo: {}", e);
            ProductCreationError::DatabaseError
        })


}

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
        .map_err(|e| {
            eprintln!("Error occured while fetching all products in repositories/product_repo: {}", e);
            FetchProductError::DatabaseError
        })

}

pub async fn fetch_product_by_id(pool: &Pool<Postgres>, product_id: Uuid) -> Result<Option<Product>, FetchProductError> {

    let query = r#"
        SELECT * FROM products
        WHERE id = $1
        AND status = 'active' AND deleted_at IS NULL
        "#;

    sqlx::query_as::<_, Product>(query)
        .bind(&product_id)
        .fetch_optional(pool).await
        .map_err(|e| {
            eprintln!("Error occurred while fetching one product by 'id = {product_id}' in repositories/product_repo: {}", e);
            FetchProductError::DatabaseError
        })
}