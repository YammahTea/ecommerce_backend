use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::{Pool, Postgres};
use crate::models::user::RegisterRequest;
use crate::services::user_service::register_user;

pub async fn register(State(pool): State<Pool<Postgres>>,
                Json(user_payload): Json<RegisterRequest>) -> impl IntoResponse {

    let result = register_user(pool, user_payload.email, user_payload.password).await;

    match result {
        Ok(success_message) => (StatusCode::OK, success_message).into_response(),

        Err(error_message) => {
            if error_message.ends_with("exists.") {
                (StatusCode::CONFLICT, error_message).into_response()
            }
            else {
                (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
            }
        }
    }

}