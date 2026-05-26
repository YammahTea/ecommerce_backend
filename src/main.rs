pub mod models;
pub mod services;
pub mod repositories;
pub mod handlers;
pub mod middleware;
pub mod errors;

use std::sync::Arc;
use axum::{Router, routing::get, http};
use axum::body::Body;
use axum::extract::Request;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use axum::http::StatusCode;
use axum::middleware::{from_fn, from_fn_with_state};
use axum::response::{IntoResponse, Response};
use sqlx::{Executor, Pool, Postgres};
use axum::routing::{delete, patch, post};
use std::time::Duration;
use deadpool_redis::{Config, Runtime};
use tracing::{error, info, instrument, Span};
use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, Layer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tower_http::{trace::{TraceLayer, DefaultMakeSpan}};
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::sensitive_headers::{SetSensitiveRequestHeadersLayer, SetSensitiveResponseHeadersLayer};
use http::header;
use crate::handlers::auth_handler::{login, register, token_refresh};
use crate::handlers::cart_handler::{add_to_cart, get_cart};
use crate::handlers::product_handler::{create_product, delete_product, get_all_products, get_product, update_product};
use crate::middleware::admin::require_admin;
use crate::middleware::auth::auth_middleware;
use crate::models::auth::AuthConfig;
use crate::models::config::{AppState, DatabaseConfig, RedisConfig};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let _guard = init_tracing();

    let database_config: DatabaseConfig = DatabaseConfig::default();
    let db_pool: Pool<Postgres> = create_pool_from_env(&database_config).await;
    info!("Successfully connected to database!");

    let redis_config = RedisConfig::default();
    // note, the redis pool is NOT async due to
    // actual connection happens on first .get().await call)
    let redis_pool = create_redis_pool(&redis_config);
    info!("Successfully created redis pool!");

    let auth_config: AuthConfig = AuthConfig::default();

    let state = AppState {
        db_pool,
        redis_pool,
        auth_config
    };

    let app = add_tracing_layer(app(state));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());
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
        .nest("/admin", admin_routes)
        .with_state(state)
}

fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_check))
        .route("/user/cart", post(add_to_cart).get(get_cart))
}

fn unprotected_routes() -> Router<AppState> {
    Router::new()
        .route("/user/register", post(register))
        .route("/user/login", post(login))
        .route("/user/refresh", post(token_refresh))
        
        .route("/products", get(get_all_products))
        .route("/products/{id}", get(get_product))
}

fn admin_routes() -> Router<AppState> {
    Router::new()
        .route("/products", post(create_product))
        .route("/products/{id}", patch(update_product))
        .route("/products/{id}", delete(delete_product))
}

async fn health_check() -> impl IntoResponse {
    // more checks will be added as the app develops
    (StatusCode::OK, "Looks good").into_response()
}

// Connect to database AND create connection pool
#[instrument(skip(config))]
async fn create_pool_from_env(config: &DatabaseConfig) -> Pool<Postgres> {

    info!("Connecting to database and creating connection pool...");

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

// Creat redis pool
#[instrument(skip(redis_config))]
fn create_redis_pool(redis_config: &RedisConfig) -> deadpool_redis::Pool {

    info!("Creating redis pool...");

    let config = Config::from_url(&redis_config.redis_url);

    config.create_pool(Some(Runtime::Tokio1)).expect("Failed to create redis pool")
}

fn init_tracing() -> impl Drop {
    let file_appender = rolling::daily("logs/", "ecommerce");
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

    let terminal_layer = tracing_subscriber::fmt::layer()
        .with_filter(EnvFilter::from_default_env());

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(file_writer)
        .with_ansi(false)
        .with_filter(EnvFilter::new("error"));

    tracing_subscriber::registry()
        .with(terminal_layer)
        .with(file_layer)
        .init();

    guard
}

fn add_tracing_layer(router: Router) -> Router {

    let headers: Arc<[_]> = Arc::new([
        header::AUTHORIZATION,
        header::PROXY_AUTHENTICATE,
        header::COOKIE,
        header::SET_COOKIE,
    ]);

    router
        .layer(SetSensitiveRequestHeadersLayer::from_shared(Arc::clone(&headers)))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new().include_headers(true)
                )
                .on_request(|request: &Request<Body>, _span: &Span| {
                    info!("request: {} {}", request.method(), request.uri().path())

                })            .on_response(|response: &Response<Body>, latency: Duration, _span: &Span|{
                    info!("response: {}, latency {:?}", response.status(), latency)
                })
                .on_failure(|error: ServerErrorsFailureClass, _latency: Duration, _span: &Span|{
                    error!(error = ?error, latency = ?_latency, "Error")
                })

        )
        .layer(SetSensitiveResponseHeadersLayer::from_shared(headers))

}