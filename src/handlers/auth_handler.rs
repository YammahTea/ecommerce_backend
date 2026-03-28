use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::{Pool, Postgres};
use email_address::EmailAddress;
use crate::models::user::RegisterRequest;
use crate::services::user_service::register_user;

pub async fn register(State(pool): State<Pool<Postgres>>,
                Json(user_payload): Json<RegisterRequest>) -> impl IntoResponse {

    let is_valid_email = EmailAddress::is_valid(user_payload.email.as_ref());

    if is_valid_email {
        let result = register_user(pool, String::from(user_payload.email), user_payload.password).await;

        match result {
            Ok(success_message) => (StatusCode::CREATED, success_message).into_response(),

            // TODO: Implement Enum (App error)
            Err(error_message) => {
                if error_message.ends_with("exists.") {
                    (StatusCode::CONFLICT, error_message).into_response()
                }
                else {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error happened while trying to create the user.").into_response()
                }
            }
        }
    }

    else {
        (StatusCode::BAD_REQUEST, "Not a valid email address").into_response()
    }

}