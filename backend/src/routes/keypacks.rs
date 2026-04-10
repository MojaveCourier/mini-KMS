use axum::{Json, extract::State};

use crate::{
    error::AppError,
    models::{common::ApiResponse, keypack::KeypackDto},
    state::AppState,
};

pub async fn list_keypacks(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<KeypackDto>>>, AppError> {
    let keypacks = state.keypack_service.list_keypacks().await?;
    Ok(Json(ApiResponse::ok(keypacks)))
}
