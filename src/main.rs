use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let app: Router = app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn app () -> Router {
    Router::new()
        .route("/health", get(health_check))
}

async fn health_check() -> impl IntoResponse {
    // more check ups will be added as the app develops
    (StatusCode::OK, "Looks good").into_response()
}
