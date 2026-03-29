pub mod models;
pub mod services;
pub mod repositories;
pub mod handlers;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::{Executor, Pool, Postgres};
use axum::routing::post;
use crate::handlers::auth_handler::{login, register};
use crate::models::database_config::DatabaseConfig;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = DatabaseConfig::default();
    let pool: Pool<Postgres> = create_pool_from_env(&config).await;
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
        .route("/user/login", post(login))
        .with_state(pool)
}

async fn health_check() -> impl IntoResponse {
    // more checks will be added as the app develops
    (StatusCode::OK, "Looks good").into_response()
}

// Connect to database
async fn create_pool_from_env(config: &DatabaseConfig) -> Pool<Postgres> {

    println!("Connecting to database...");

    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)

        .test_before_acquire(true)

        .acquire_timeout(config.acquire_timeout)

        .after_connect(|conn, _meta| Box::pin(async move {
            conn.execute("SELECT 1;").await?;
            Ok(())
        }))

        .idle_timeout(config.idle_timeout)

        .connect(&*config.database_url)
        .await
        .expect("Failed to connect to database, please check if the database is running or if the database url is correct")

}