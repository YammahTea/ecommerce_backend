use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use email_address::EmailAddress;
use tracing::instrument;
use crate::models::config::AppState;
use crate::errors::user_error::{UserCreationError, UserLoginError};
use crate::models::user::{LoginRequest, RefreshTokenRequest, RegisterRequest};
use crate::services::user_service::{login_user, register_user, token_rotation};

// NOTE: using instrument, please skip everything, fields are logged in the "services" layer ONLY

#[instrument(skip(state, user_payload))]
pub async fn register(State(state): State<AppState>,
                Json(user_payload): Json<RegisterRequest>) -> Result<impl IntoResponse, UserCreationError> {

    let is_valid_email = EmailAddress::is_valid(user_payload.email.as_ref());

    if is_valid_email {

        let result = register_user(&state.db_pool, state.auth_config, String::from(user_payload.email), user_payload.password).await;

        match result {
            Ok(success_message) => Ok((StatusCode::CREATED, success_message).into_response()),
            Err(e) => Err(e)
        }
    }

    else { Err(UserCreationError::InvalidEmail) }

}

#[instrument(skip(state, user_payload))]
pub async fn login(State(state): State<AppState>,
                   Json(user_payload): Json<LoginRequest>) -> Result<impl IntoResponse, UserLoginError> {
   
    let tokens = login_user(&state.db_pool, &state.auth_config, user_payload.identifier, user_payload.password).await?;
    Ok((StatusCode::OK, Json(tokens)).into_response())
}

pub async fn token_refresh(State(state): State<AppState>,
                Json(raw_user_refresh_token): Json<RefreshTokenRequest>)  -> Result<impl IntoResponse, UserLoginError> {
    
    let new_tokens = token_rotation(&state.db_pool, &raw_user_refresh_token.raw_refresh_token, &state.auth_config).await?;
    Ok((StatusCode::OK, Json(new_tokens)).into_response())

}