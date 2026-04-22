use axum::extract::{Request};
use axum::middleware::Next;
use axum::response::IntoResponse;
use crate::errors::middleware_error::AuthorizationMiddlewareError;
use crate::models::auth::Claims;

pub async fn require_admin(request: Request, next: Next)
                           -> Result<impl IntoResponse, AuthorizationMiddlewareError> {

    let claims =
        request.extensions().get::<Claims>()
        .ok_or(AuthorizationMiddlewareError::InsufficientPermissions)?;

    if claims.role == "admin" {
        Ok(next.run(request).await)
    }
    else {
        Err(AuthorizationMiddlewareError::InsufficientPermissions)
    }
}