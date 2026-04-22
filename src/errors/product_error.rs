use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

// NOTE: Any "DatabaseError" catches ALL errors to connection issues or unexpected errors

#[derive(Debug)]
pub enum ProductCreationError {
    // More errors will be added as the product table is full
    InvalidPrice,
    InvalidStockQuantity,
    InvalidName,
    InvalidDescription,
    DatabaseError
}

impl IntoResponse for ProductCreationError {
    fn into_response(self) -> Response {
        let body = match self {

            ProductCreationError::InvalidPrice => (StatusCode::BAD_REQUEST, "Price can not be negative."),
            ProductCreationError::InvalidStockQuantity => (StatusCode::BAD_REQUEST, "Stock quantity can not be negative."),
            ProductCreationError::InvalidName => (StatusCode::BAD_REQUEST, "Product name can not be empty"),
            ProductCreationError::InvalidDescription=> (StatusCode::BAD_REQUEST, "Product description can not be empty"),
            ProductCreationError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong while creating the product.")

        };

        body.into_response()
    }
}