use axum::{Json, extract::{Path, State}};

use crate::{
    error::AppError,
    models::{common::ApiResponse, device::DeviceRecord},
    state::AppState,
};

pub async fn list_devices(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<DeviceRecord>>>, AppError> {
    let devices = state.device_service.list_devices().await?;
    Ok(Json(ApiResponse::ok(devices)))
}

pub async fn get_device(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<DeviceRecord>>, AppError> {
    let device = state.device_service.get_device(id).await?;
    Ok(Json(ApiResponse::ok(device)))
}
