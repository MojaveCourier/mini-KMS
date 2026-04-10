use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder, Set};

use crate::entity::audit_logs;

pub async fn append(
    db: &DatabaseConnection,
    user_id: Option<i64>,
    action: &str,
    target_type: &str,
    target_id: Option<i64>,
    detail: Option<String>,
) -> Result<audit_logs::Model, sea_orm::DbErr> {
    audit_logs::ActiveModel {
        user_id: Set(user_id),
        action: Set(action.to_string()),
        target_type: Set(target_type.to_string()),
        target_id: Set(target_id),
        detail: Set(detail),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(db)
    .await
}

pub async fn list(db: &DatabaseConnection) -> Result<Vec<audit_logs::Model>, sea_orm::DbErr> {
    audit_logs::Entity::find()
        .order_by_desc(audit_logs::Column::CreatedAt)
        .all(db)
        .await
}
