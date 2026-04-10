use std::sync::Arc;

use axum::{Extension, Json, extract::{Path, State}};

use crate::{
    error::AppError,
    models::{
        auth::AuthClaims,
        common::ApiResponse,
        device::{CreateDeviceRequest, DeviceRecord, UpdateDeviceRequest},
    },
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

pub async fn create_device(
    State(state): State<AppState>,
    Extension(claims): Extension<Arc<AuthClaims>>,
    Json(payload): Json<CreateDeviceRequest>,
) -> Result<Json<ApiResponse<DeviceRecord>>, AppError> {
    let device = state.device_service.create_device(claims.sub, payload).await?;
    Ok(Json(ApiResponse::ok(device)))
}

pub async fn update_device(
    State(state): State<AppState>,
    Extension(claims): Extension<Arc<AuthClaims>>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateDeviceRequest>,
) -> Result<Json<ApiResponse<DeviceRecord>>, AppError> {
    let device = state.device_service.update_device(claims.sub, id, payload).await?;
    Ok(Json(ApiResponse::ok(device)))
}

pub async fn delete_device(
    State(state): State<AppState>,
    Extension(claims): Extension<Arc<AuthClaims>>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<bool>>, AppError> {
    state.device_service.delete_device(claims.sub, id).await?;
    Ok(Json(ApiResponse::ok(true)))
}
