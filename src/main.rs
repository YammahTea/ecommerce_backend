pub mod models;
pub mod services;
pub mod repositories;
pub mod handlers;
pub mod middleware;
pub mod errors;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use axum::http::StatusCode;
use axum::middleware::{from_fn, from_fn_with_state};
use axum::response::IntoResponse;
use sqlx::{Executor, Pool, Postgres};
use axum::routing::post;
use crate::handlers::auth_handler::{login, register};
use crate::handlers::product_handler::create_product;
use crate::middleware::admin::require_admin;
use crate::middleware::auth::auth_middleware;
use crate::models::auth::AuthConfig;
use crate::models::config::{AppState, DatabaseConfig};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_config: DatabaseConfig = DatabaseConfig::default();
    let pool: Pool<Postgres> = create_pool_from_env(&database_config).await;
    let auth_config: AuthConfig = AuthConfig::default();
    let state = AppState { pool, auth_config };

    println!("Successfully connected to database!");

    let app = app(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}


fn app (state: AppState) -> Router {
    let protected_routes = protected_routes()
        .layer(from_fn_with_state(state.clone(), auth_middleware));

    let unprotected_routes = unprotected_routes();

    let admin_routes = admin_routes()
        .layer(from_fn(require_admin))
        .layer(from_fn_with_state(state.clone(), auth_middleware));

    Router::new()
        .merge(protected_routes)
        .merge(unprotected_routes)
        .merge(admin_routes)
        .with_state(state)
}

fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_check))
}

fn unprotected_routes() -> Router<AppState> {
    Router::new()
        .route("/user/register", post(register))
        .route("/user/login", post(login))
}

fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/products", post(create_product)) // Create products
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