use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
};

use crate::entity::users;

pub async fn find_by_account(db: &DatabaseConnection, account: &str) -> Result<Option<users::Model>, sea_orm::DbErr> {
    users::Entity::find()
        .filter(users::Column::Account.eq(account))
        .one(db)
        .await
}

pub async fn find_by_id(db: &DatabaseConnection, user_id: i64) -> Result<Option<users::Model>, sea_orm::DbErr> {
    users::Entity::find_by_id(user_id).one(db).await
}

pub async fn list(db: &DatabaseConnection) -> Result<Vec<users::Model>, sea_orm::DbErr> {
    users::Entity::find().all(db).await
}

pub async fn count(db: &DatabaseConnection) -> Result<u64, sea_orm::DbErr> {
    users::Entity::find().count(db).await
}

pub async fn create(
    db: &DatabaseConnection,
    account: String,
    password_hash: String,
    role: String,
) -> Result<users::Model, sea_orm::DbErr> {
    users::ActiveModel {
        account: Set(account),
        password_hash: Set(password_hash),
        role: Set(role),
        created_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(db)
    .await
}

pub async fn update(
    db: &DatabaseConnection,
    model: users::Model,
    account: Option<String>,
    password_hash: Option<String>,
    role: Option<String>,
) -> Result<users::Model, sea_orm::DbErr> {
    let mut active: users::ActiveModel = model.into();
    if let Some(account) = account {
        active.account = Set(account);
    }
    if let Some(password_hash) = password_hash {
        active.password_hash = Set(password_hash);
    }
    if let Some(role) = role {
        active.role = Set(role);
    }
    active.update(db).await
}

pub async fn delete(db: &DatabaseConnection, model: users::Model) -> Result<(), sea_orm::DbErr> {
    let active: users::ActiveModel = model.into();
    active.delete(db).await?;
    Ok(())
}
