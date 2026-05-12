use std::env;
use dotenvy::dotenv;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use fake::{Dummy, Fake, Faker};
use fake::faker::lorem::en::Sentence;
use fake::faker::company::en::CatchPhrase;

// NOTE: this file is just for generating fake data in the database for testing purposes
// the logic is just built to connect to the database and is simpler
// because this file is NOT considered as a part of the project's main purpose!

// 1- To run this file: cargo run --bin seed
// (Make sure that the database is running and you ran 'sqlx migrate run')
// 2- To run the main application:
// run this 'cargo watch -x "run --bin ecommerce_backend"
// 3- To automate running the application to be the 'ecommerce_backend'
// update the Cargo.toml by adding:
// default-run = "ecommerce_backend"   # beneath [package]

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_pool: Pool<Postgres> = create_pool_from_env().await;

    println!("Connected to db");

    for _ in 0..100 {
        let product: CreateProduct = Faker.fake();
        insert_product(&db_pool, &product).await;
    }

    println!("Finished creating random products");

}

async fn create_pool_from_env() -> Pool<Postgres> {

    let database_url =  env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");

    PgPoolOptions::new()

        .test_before_acquire(true)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database, please check if the database is running or if the database url is correct")

}

// ============================= Product Repo =======================================

async fn insert_product(db_pool: &Pool<Postgres>, product: &CreateProduct) {

    sqlx::query!("
        INSERT INTO products
        (name, description, price_in_cents, stock_quantity, status)
        VALUES
        ($1, $2, $3, $4, $5)",
        product.name,
        product.description,
        product.price_in_cents,
        product.stock_quantity,
        product.status.clone() as ProductStatus   // the cast is needed for the custom sqlx::Type enums
    )
        .execute(db_pool)
        .await
        .expect("Failed to insert product");

}


// ============================= Product structs =======================================

#[derive(Debug, Clone, Dummy, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[sqlx(rename_all = "lowercase")]   // DB boundary: stored as "draft"/"active"/"archived" to match CHECK constraint
pub enum ProductStatus {
    Draft,
    Active,
    Archived,
}

#[derive(Debug, Dummy)]
pub struct CreateProduct {
    #[dummy(faker = "CatchPhrase()")]
    name: String,

    #[dummy(faker = "Sentence(5..10)")]
    description: String,

    #[dummy(faker = "100..5000")]
    price_in_cents: i32,

    #[dummy(faker = "1..100")]
    stock_quantity: i32,

    status: ProductStatus
}
