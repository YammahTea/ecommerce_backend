use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum UserCreationError {
    InvalidEmail,
    UserAlreadyExists,
    HashingError,
    DatabaseError
}

impl IntoResponse for UserCreationError {
    fn into_response(self) -> Response {

        let body = match self {
            UserCreationError::InvalidEmail => (StatusCode::BAD_REQUEST, "Invalid email address."),
            UserCreationError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists."),
            UserCreationError::HashingError => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong while hashing the password."),
            UserCreationError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to connect to database.")
        };

        body.into_response()
    }
}