use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::{Pool, Postgres};
use email_address::EmailAddress;
use crate::models::error::{UserCreationError, UserLoginError};
use crate::models::user::{LoginRequest, RegisterRequest};
use crate::services::user_service::{login_user, register_user};

pub async fn register(State(pool): State<Pool<Postgres>>,
                Json(user_payload): Json<RegisterRequest>) -> Result<impl IntoResponse, UserCreationError> {

    let is_valid_email = EmailAddress::is_valid(user_payload.email.as_ref());

    if is_valid_email {
        let result = register_user(pool, String::from(user_payload.email), user_payload.password).await;

        match result {
            Ok(success_message) => Ok((StatusCode::CREATED, success_message).into_response()),
            Err(e) => Err(e)
        }
    }

    else { Err(UserCreationError::InvalidEmail) }

}

pub async fn login(State(pool): State<Pool<Postgres>>,
                   Json(user_payload): Json<LoginRequest>) -> Result<impl IntoResponse, UserLoginError> {
    let success_message = login_user(pool, user_payload.identifier, user_payload.password).await?;
    Ok((StatusCode::OK, success_message).into_response())
}