pub mod models;
pub mod services;
pub mod repositories;
pub mod handlers;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::{Executor, Pool, Postgres};
use std::time::Duration;
use axum::routing::post;
use crate::handlers::auth_handler::register;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool: Pool<Postgres> = create_pool_from_env().await;
    println!("Successfully connected to database!");

    let app = app(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}


fn app (pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/user/register", post(register))
        .with_state(pool)
}

async fn health_check() -> impl IntoResponse {
    // more check ups will be added as the app develops
    (StatusCode::OK, "Looks good").into_response()
}

// Connect to database
async fn create_pool_from_env() -> Pool<Postgres> {
    let database_url = &env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");

    println!("Connecting to database...");

    PgPoolOptions::new()
        .max_connections(15)
        .test_before_acquire(true)
        .after_connect(|conn, _meta| Box::pin(async move {
            conn.execute("SELECT 1;").await?;
            Ok(())
        }))
        .idle_timeout(Duration::from_secs(300))
        .connect(&database_url)
        .await
        .expect("Failed to connect to database, please check if the database is running or if the db url is correct")

}