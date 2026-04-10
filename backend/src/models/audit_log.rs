use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct AuditLogRecord {
    pub id: i64,
    pub user_id: Option<i64>,
    pub action: String,
    pub target_type: String,
    pub target_id: Option<i64>,
    pub detail: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AuditLogDto {
    pub id: i64,
    pub user_id: Option<i64>,
    pub action: String,
    pub target_type: String,
    pub target_id: Option<i64>,
    pub detail: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<&AuditLogRecord> for AuditLogDto {
    fn from(value: &AuditLogRecord) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            action: value.action.clone(),
            target_type: value.target_type.clone(),
            target_id: value.target_id,
            detail: value.detail.clone(),
            created_at: value.created_at,
        }
    }
}
