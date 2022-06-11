use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "articles")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub category_id: i32,
    pub title: String,
    pub content: String,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub is_del: bool,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Relation {
    Category,
}

impl RelationTrait for Relation {
    fn def(&self) -> sea_orm::RelationDef {
        match self {
            Self::Category => Entity::belongs_to(super::category::Entity)
                .from(Column::CategoryId)
                .to(super::category::Column::Id)
                .into(),
        }
    }
}
impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
