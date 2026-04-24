use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use uuid::Uuid;
use crate::models::config::AppState;
use crate::errors::product_error::{FetchProductError, ProductCreationError};
use crate::models::product::{CreateProductRequest, ProductPagination};
use crate::services::product_service::{add_new_product, get_single_product, list_products};

pub async fn create_product(State(state): State<AppState>, Json(user_product_payload): Json<CreateProductRequest>)
                            -> Result<impl IntoResponse, ProductCreationError> {

    let created_product = add_new_product(&user_product_payload, &state.pool).await?;
    Ok((StatusCode::CREATED, Json(created_product)).into_response())
}

pub async fn get_all_products(State(state): State<AppState>, Query(product_pagination): Query<ProductPagination>) -> Result<impl IntoResponse, FetchProductError> {

    let products = list_products(product_pagination, &state.pool).await?;
    Ok((StatusCode::OK, Json(products)))

}

pub async fn get_product(State(state): State<AppState>, Path(product_id): Path<Uuid>) -> Result<impl IntoResponse, FetchProductError> {

    let product = get_single_product(product_id, &state.pool).await?;
    Ok((StatusCode::OK, Json(product)))

}