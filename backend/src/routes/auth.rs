use axum::{Json, extract::State};

use crate::{
    error::AppError,
    models::{auth::{LoginRequest, LoginResponse}, common::ApiResponse},
    state::AppState,
};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, AppError> {
    let response = state.auth_service.login(payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}
