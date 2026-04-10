use std::sync::Arc;

use axum::{Extension, Json, extract::{Path, State}};

use crate::{
    error::AppError,
    models::{
        auth::AuthClaims,
        common::ApiResponse,
        keypack::{CreateKeypackRequest, KeypackDto, UpdateKeypackRequest},
    },
    state::AppState,
};

pub async fn list_keypacks(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<KeypackDto>>>, AppError> {
    let keypacks = state.keypack_service.list_keypacks().await?;
    Ok(Json(ApiResponse::ok(keypacks)))
}

pub async fn create_keypack(
    State(state): State<AppState>,
    Extension(claims): Extension<Arc<AuthClaims>>,
    Json(payload): Json<CreateKeypackRequest>,
) -> Result<Json<ApiResponse<KeypackDto>>, AppError> {
    let keypack = state.keypack_service.create_keypack(claims.sub, payload).await?;
    Ok(Json(ApiResponse::ok(keypack)))
}

pub async fn update_keypack(
    State(state): State<AppState>,
    Extension(claims): Extension<Arc<AuthClaims>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateKeypackRequest>,
) -> Result<Json<ApiResponse<KeypackDto>>, AppError> {
    let keypack = state.keypack_service.update_keypack(claims.sub, id, payload).await?;
    Ok(Json(ApiResponse::ok(keypack)))
}

pub async fn delete_keypack(
    State(state): State<AppState>,
    Extension(claims): Extension<Arc<AuthClaims>>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<bool>>, AppError> {
    state.keypack_service.delete_keypack(claims.sub, id).await?;
    Ok(Json(ApiResponse::ok(true)))
}
