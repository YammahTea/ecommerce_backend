use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use tracing::instrument;
use uuid::Uuid;
use crate::models::config::AppState;
use crate::errors::product_error::{FetchProductError, CreateProductError, UpdateProductError, SoftDeleteProductError};
use crate::models::product::{CreateProductRequest, ProductPagination, UpdateProductRequest};
use crate::services::product_service::{add_new_product, edit_product, get_single_product, list_products, remove_product};

// NOTE: using instrument, please skip everything, fields are logged in the "services" layer ONLY

#[instrument(skip(state, user_product_payload))]
pub async fn create_product(State(state): State<AppState>, Json(user_product_payload): Json<CreateProductRequest>)
                            -> Result<impl IntoResponse, CreateProductError> {

    let created_product = add_new_product(&user_product_payload, &state.pool).await?;
    Ok((StatusCode::CREATED, Json(created_product)).into_response())
}

#[instrument(skip(state, product_pagination))]
pub async fn get_all_products(State(state): State<AppState>, Query(product_pagination): Query<ProductPagination>) 
                            -> Result<impl IntoResponse, FetchProductError> {

    let products = list_products(product_pagination, &state.pool).await?;
    Ok((StatusCode::OK, Json(products)))

}

#[instrument(skip(state, product_id))]
pub async fn get_product(State(state): State<AppState>, Path(product_id): Path<Uuid>) 
                            -> Result<impl IntoResponse, FetchProductError> {
    
    let product = get_single_product(product_id, &state.pool).await?;
    Ok((StatusCode::OK, Json(product)))

}

#[instrument(skip(state, product_id, user_product_payload))]
pub async fn update_product(State(state): State<AppState>, Path(product_id): Path<Uuid>, Json(user_product_payload): Json<UpdateProductRequest>) 
                            -> Result<impl IntoResponse, UpdateProductError> {
    
    let updated_product = edit_product(product_id, &user_product_payload, &state.pool).await?;
    Ok((StatusCode::OK, Json(updated_product)))
}

#[instrument(skip(state, product_id))]
pub async fn delete_product(State(state): State<AppState>, Path(product_id): Path<Uuid>) 
                            -> Result<impl IntoResponse, SoftDeleteProductError> {
    
    let success = remove_product(product_id, &state.pool).await?;
    Ok((StatusCode::OK, success))
}