use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter};

use crate::entity::devices;

pub async fn list(db: &DatabaseConnection) -> Result<Vec<devices::Model>, sea_orm::DbErr> {
    devices::Entity::find().all(db).await
}

pub async fn find_by_id(db: &DatabaseConnection, device_id: i64) -> Result<Option<devices::Model>, sea_orm::DbErr> {
    devices::Entity::find_by_id(device_id).one(db).await
}

pub async fn count(db: &DatabaseConnection) -> Result<u64, sea_orm::DbErr> {
    devices::Entity::find().count(db).await
}

pub async fn count_active(db: &DatabaseConnection) -> Result<u64, sea_orm::DbErr> {
    devices::Entity::find()
        .filter(devices::Column::Status.eq("active"))
        .count(db)
        .await
}
