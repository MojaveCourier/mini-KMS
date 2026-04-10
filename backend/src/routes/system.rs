use axum::{Json, extract::State};

use crate::{
    error::AppError,
    models::{common::ApiResponse, system::SystemStatusDto},
    state::AppState,
};

pub async fn get_status(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SystemStatusDto>>, AppError> {
    let stats = state.system_service.get_status().await?;
    Ok(Json(ApiResponse::ok(stats)))
}
