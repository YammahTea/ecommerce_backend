use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::IntoResponse;
use crate::models::config::AppState;
use crate::errors::middleware_error::{AuthenticationMiddlewareError};
use crate::services::user_service::verify_access_token;

pub async fn auth_middleware(State(state): State<AppState>, mut request: Request, next: Next) -> Result<impl IntoResponse, AuthenticationMiddlewareError> {

    // Find the authorization header in the request
    let header = request.headers().get("authorization").ok_or(AuthenticationMiddlewareError::MissingCredentials)?;

    let auth_header = header.to_str().map_err(|error_message| {
        eprintln!("Error occurred while converting header to str in middleware/auth.rs: {}", error_message);
        AuthenticationMiddlewareError::InvalidToken
    })?;

    let clean_token = auth_header.strip_prefix("Bearer ").ok_or(AuthenticationMiddlewareError::InvalidToken)?;

    let claims = verify_access_token(clean_token, &state.auth_config)?;
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)

}