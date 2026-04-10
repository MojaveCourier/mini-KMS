use sea_orm::{DatabaseConnection, EntityTrait};

use crate::entity::{devices, keypacks};

pub async fn list_with_device(
    db: &DatabaseConnection,
) -> Result<Vec<(keypacks::Model, Option<devices::Model>)>, sea_orm::DbErr> {
    keypacks::Entity::find().find_also_related(devices::Entity).all(db).await
}

pub async fn count(db: &DatabaseConnection) -> Result<u64, sea_orm::DbErr> {
    use sea_orm::PaginatorTrait;
    keypacks::Entity::find().count(db).await
}
