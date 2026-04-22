use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum AuthenticationMiddlewareError {
    MissingCredentials, // No token
    InvalidToken // Token expired
}

impl IntoResponse for AuthenticationMiddlewareError {
    fn into_response(self) -> Response {
        let body = match self {

            AuthenticationMiddlewareError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token."),
            AuthenticationMiddlewareError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials.")

        };

        body.into_response()
    }
}

#[derive(Debug)]
pub enum AuthorizationMiddlewareError {
    InsufficientPermissions
}

impl IntoResponse for AuthorizationMiddlewareError {
    fn into_response(self) -> Response {
        let body = match self {
            AuthorizationMiddlewareError::InsufficientPermissions => (StatusCode::FORBIDDEN, "Access denied.")
        };

        body.into_response()
    }
}