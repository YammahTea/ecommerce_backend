use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::{Pool, Postgres};
use email_address::EmailAddress;
use crate::models::error::UserCreationError;
use crate::models::user::RegisterRequest;
use crate::services::user_service::register_user;

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