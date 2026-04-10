use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "devices")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub serial: String,
    pub name: String,
    pub status: String,
    pub last_seen_at: Option<DateTime>,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::keypacks::Entity")]
    Keypacks,
}

impl Related<super::keypacks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Keypacks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
