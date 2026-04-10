use std::sync::Arc;

use axum::{Extension, Json, extract::{Path, State}};

use crate::{
    error::AppError,
    models::{auth::AuthClaims, common::ApiResponse, user::{CreateUserRequest, UpdateUserRequest, UserDto}},
    state::AppState,
};

pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<UserDto>>>, AppError> {
    let users = state.user_service.list_users().await?;
    Ok(Json(ApiResponse::ok(users)))
}

pub async fn create_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Arc<AuthClaims>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<ApiResponse<UserDto>>, AppError> {
    let user = state.user_service.create_user(claims.sub, payload).await?;
    Ok(Json(ApiResponse::ok(user)))
}

pub async fn update_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Arc<AuthClaims>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<UserDto>>, AppError> {
    let user = state.user_service.update_user(claims.sub, id, payload).await?;
    Ok(Json(ApiResponse::ok(user)))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Arc<AuthClaims>>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<bool>>, AppError> {
    state.user_service.delete_user(claims.sub, id).await?;
    Ok(Json(ApiResponse::ok(true)))
}
