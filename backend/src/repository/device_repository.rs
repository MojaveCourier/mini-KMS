use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
};

use crate::entity::devices;

pub async fn list(db: &DatabaseConnection) -> Result<Vec<devices::Model>, sea_orm::DbErr> {
    devices::Entity::find().all(db).await
}

pub async fn find_by_id(db: &DatabaseConnection, device_id: i64) -> Result<Option<devices::Model>, sea_orm::DbErr> {
    devices::Entity::find_by_id(device_id).one(db).await
}

pub async fn find_by_serial(db: &DatabaseConnection, serial: &str) -> Result<Option<devices::Model>, sea_orm::DbErr> {
    devices::Entity::find()
        .filter(devices::Column::Serial.eq(serial))
        .one(db)
        .await
}

pub async fn create(
    db: &DatabaseConnection,
    serial: String,
    name: String,
    status: String,
    last_seen_at: Option<chrono::NaiveDateTime>,
) -> Result<devices::Model, sea_orm::DbErr> {
    devices::ActiveModel {
        serial: Set(serial),
        name: Set(name),
        status: Set(status),
        last_seen_at: Set(last_seen_at),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(db)
    .await
}

pub async fn update(
    db: &DatabaseConnection,
    model: devices::Model,
    serial: String,
    name: String,
    status: String,
    last_seen_at: Option<chrono::NaiveDateTime>,
) -> Result<devices::Model, sea_orm::DbErr> {
    let mut active: devices::ActiveModel = model.into();
    active.serial = Set(serial);
    active.name = Set(name);
    active.status = Set(status);
    active.last_seen_at = Set(last_seen_at);
    active.update(db).await
}

pub async fn delete(db: &DatabaseConnection, model: devices::Model) -> Result<(), sea_orm::DbErr> {
    let active: devices::ActiveModel = model.into();
    active.delete(db).await?;
    Ok(())
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
