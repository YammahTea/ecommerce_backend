use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sqlx::Error;
// NOTE: Any "DatabaseError" catches ALL errors to connection issues or unexpected errors

#[derive(Debug)]
pub enum ProductValidationError {
    // More errors will be added as the product table grows
    InvalidName,
    InvalidDescription,
    InvalidPrice,
    InvalidStockQuantity,
}

impl IntoResponse for ProductValidationError {
    fn into_response(self) -> Response {
        let body = match self {

            ProductValidationError::InvalidName => (StatusCode::BAD_REQUEST, "Product name cannot be empty"),
            ProductValidationError::InvalidDescription=> (StatusCode::BAD_REQUEST, "Product description cannot be empty"),
            ProductValidationError::InvalidPrice => (StatusCode::BAD_REQUEST, "Price cannot be negative."),
            ProductValidationError::InvalidStockQuantity => (StatusCode::BAD_REQUEST, "Stock quantity cannot be negative.")

        };

        body.into_response()
    }
}


#[derive(Debug)]
pub enum CreateProductError {
    ValidationError(ProductValidationError),
    DatabaseError
}

impl IntoResponse for CreateProductError {
    fn into_response(self) -> Response {
        match self {

            CreateProductError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong while creating the product.").into_response(),
            CreateProductError::ValidationError(e) => e.into_response()
        }

    }
}

impl From<ProductValidationError> for CreateProductError {
    fn from(value: ProductValidationError) -> Self {
        CreateProductError::ValidationError(value)
    }
}

#[derive(Debug)]
pub enum UpdateProductError {
    ValidationError(ProductValidationError),
    ProductNotFound,
    DatabaseError
}

impl IntoResponse for UpdateProductError {
    fn into_response(self) -> Response {

        match self {
            UpdateProductError::ProductNotFound => (StatusCode::NOT_FOUND, "Cannot update: product does not exist.").into_response(),
            UpdateProductError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong while updating the product.").into_response(),
            UpdateProductError::ValidationError(e) => e.into_response()
        }
    }
}

impl From<ProductValidationError> for UpdateProductError {
    fn from(value: ProductValidationError) -> Self {
        UpdateProductError::ValidationError(value)
    }
}

#[derive(Debug)]
pub enum SoftDeleteProductError {
    ProductNotFound,
    DatabaseError
}

impl IntoResponse for SoftDeleteProductError {
    fn into_response(self) -> Response {

        match self {
            SoftDeleteProductError::ProductNotFound => (StatusCode::NOT_FOUND, "Cannot soft delete: product does not exist").into_response(),
            SoftDeleteProductError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong while soft deleting the product.").into_response(),
        }
    }
}

impl From<sqlx::Error> for SoftDeleteProductError {
    fn from(_val: Error) -> Self {
        SoftDeleteProductError::DatabaseError
    }
}

pub enum FetchProductError {
    ProductNotFound,
    DatabaseError
}

impl IntoResponse for FetchProductError {
    fn into_response(self) -> Response {
        let body = match self {
            FetchProductError::ProductNotFound => (StatusCode::NOT_FOUND, "Product ID does not exist."),
            FetchProductError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong while fetching products.")
        };

        body.into_response()
    }
}