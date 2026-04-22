use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::models::config::AppState;
use crate::errors::product_error::ProductCreationError;
use crate::models::product::CreateProductRequest;
use crate::services::product_service::add_new_product;

pub async fn create_product(State(state): State<AppState>, Json(user_product_payload): Json<CreateProductRequest>)
                            -> Result<impl IntoResponse, ProductCreationError> {

    let created_product = add_new_product(&user_product_payload, &state.pool).await?;
    Ok((StatusCode::CREATED, Json(created_product)).into_response())
}
