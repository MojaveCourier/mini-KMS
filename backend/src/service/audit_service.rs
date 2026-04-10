use chrono::{DateTime, Utc};
use sea_orm::DatabaseConnection;

use crate::{error::AppError, models::audit_log::AuditLogDto, repository::audit_log_repository};

#[derive(Clone)]
pub struct AuditService {
    db: DatabaseConnection,
}

impl AuditService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list_logs(&self) -> Result<Vec<AuditLogDto>, AppError> {
        let logs = audit_log_repository::list(&self.db)
            .await
            .map_err(|_| AppError::internal("failed to load audit logs"))?;

        Ok(logs
            .into_iter()
            .map(|log| AuditLogDto {
                id: log.id,
                user_id: log.user_id,
                action: log.action,
                target_type: log.target_type,
                target_id: log.target_id,
                detail: log.detail,
                created_at: DateTime::<Utc>::from_naive_utc_and_offset(log.created_at, Utc),
            })
            .collect())
    }
}
