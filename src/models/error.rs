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

#[derive(Debug)]
pub enum UserLoginError {
    InvalidCredentials, // username or password is incorrect or user does NOT exist
    TokenCreationError,
    DatabaseError
}

impl IntoResponse for UserLoginError {
    fn into_response(self) -> Response {
        let body = match self {

            UserLoginError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Wrong username or password."),
            UserLoginError::TokenCreationError => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong while generating the token."),
            UserLoginError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong while fetching the user.")

        };

        body.into_response()
    }
}


#[derive(Debug)]
pub enum AuthMiddlewareError {
    MissingCredentials, // No token
    InvalidToken // Token expired
}

impl IntoResponse for AuthMiddlewareError {
    fn into_response(self) -> Response {
        let body = match self {

            AuthMiddlewareError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token."),
            AuthMiddlewareError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials.")

        };

        body.into_response()
    }
}