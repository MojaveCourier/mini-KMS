use axum::{Json, extract::State};

use crate::{
    error::AppError,
    models::{audit_log::AuditLogDto, common::ApiResponse},
    state::AppState,
};

pub async fn list_audit_logs(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<AuditLogDto>>>, AppError> {
    let logs = state.audit_service.list_logs().await?;
    Ok(Json(ApiResponse::ok(logs)))
}
