use std::collections::HashMap;
use axum::extract::{State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum::response::IntoResponse;
use tracing::{debug, instrument};
use crate::errors::cart_error::CartError;
use crate::models::auth::Claims;
use crate::models::config::AppState;
use crate::models::cart::AddToCartRequest;
use crate::services::cart_service::{add_items_to_cart, get_items_from_cart};

#[instrument(skip(state, user_items_to_cart_payload, claims))]
pub async fn add_to_cart(State(state): State<AppState>, Extension(claims): Extension<Claims>, Json(user_items_to_cart_payload): Json<AddToCartRequest>)
                     -> Result<impl IntoResponse, CartError> {

    debug!("before item_status");

    let items_status: (HashMap<String, i32>, HashMap<String, i32>) = add_items_to_cart(
        &state.redis_pool,
        &state.db_pool,
        &&claims.sub,
        user_items_to_cart_payload.items
    ).await?;

    debug!("Passed add_items_to_Cart");
    debug!(items_status = ?items_status, "Items status");

    Ok((StatusCode::OK, Json(items_status)).into_response())

}

#[instrument(skip(state, claims))]
pub async fn get_cart(State(state): State<AppState>, Extension(claims): Extension<Claims>) -> Result<impl IntoResponse, CartError> {

    let items_in_cart = get_items_from_cart(&state.redis_pool, state.db_pool, &claims.sub).await?;
    Ok((StatusCode::OK, Json(items_in_cart)).into_response())
}