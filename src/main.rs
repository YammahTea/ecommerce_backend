use axum::{Router, routing::get};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::{Pool, Postgres};

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
        .connect(&database_url)
        .await
        .expect("Failed to connect to database, please check if the database is running or if the db url is correct")

}