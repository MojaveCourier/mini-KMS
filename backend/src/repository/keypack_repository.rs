use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
};

use crate::entity::{devices, keypacks};

pub async fn list_with_device(
    db: &DatabaseConnection,
) -> Result<Vec<(keypacks::Model, Option<devices::Model>)>, sea_orm::DbErr> {
    keypacks::Entity::find().find_also_related(devices::Entity).all(db).await
}

pub async fn find_by_id(db: &DatabaseConnection, keypack_id: i64) -> Result<Option<keypacks::Model>, sea_orm::DbErr> {
    keypacks::Entity::find_by_id(keypack_id).one(db).await
}

pub async fn count_by_device(db: &DatabaseConnection, device_id: i64) -> Result<u64, sea_orm::DbErr> {
    keypacks::Entity::find()
        .filter(keypacks::Column::DeviceId.eq(device_id))
        .count(db)
        .await
}

pub async fn create(
    db: &DatabaseConnection,
    device_id: i64,
    version: String,
    status: String,
) -> Result<keypacks::Model, sea_orm::DbErr> {
    keypacks::ActiveModel {
        device_id: Set(device_id),
        version: Set(version),
        status: Set(status),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(db)
    .await
}

pub async fn update(
    db: &DatabaseConnection,
    model: keypacks::Model,
    device_id: i64,
    version: String,
    status: String,
) -> Result<keypacks::Model, sea_orm::DbErr> {
    let mut active: keypacks::ActiveModel = model.into();
    active.device_id = Set(device_id);
    active.version = Set(version);
    active.status = Set(status);
    active.update(db).await
}

pub async fn delete(db: &DatabaseConnection, model: keypacks::Model) -> Result<(), sea_orm::DbErr> {
    let active: keypacks::ActiveModel = model.into();
    active.delete(db).await?;
    Ok(())
}

pub async fn count(db: &DatabaseConnection) -> Result<u64, sea_orm::DbErr> {
    keypacks::Entity::find().count(db).await
}
