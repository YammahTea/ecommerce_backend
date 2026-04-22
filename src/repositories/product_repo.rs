use sqlx::{Pool, Postgres};
use crate::errors::product_error::{ProductCreationError};
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