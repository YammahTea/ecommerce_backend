use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

// NOTE: Any "DatabaseError" catches ALL errors to connection issues or unexpected errors

#[derive(Debug)]
pub enum CartError {
    UserNotFound,
    CartNotFound, // cart is empty or doesn't exist
    RedisKeyError,
    DatabaseError,
    RedisConnectionError,
}

impl IntoResponse for CartError {
    fn into_response(self) -> Response {

        let body = match self {
            CartError::UserNotFound => (StatusCode::NOT_FOUND, "User not found."),
            CartError::CartNotFound => (StatusCode::NOT_FOUND, "Cart is empty."),
            CartError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR,  "Something went wrong while connecting to database."),
            CartError::RedisConnectionError => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong while connecting to cache."),
            CartError::RedisKeyError => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong while accessing the cart."),
        };

        body.into_response()
    }
}