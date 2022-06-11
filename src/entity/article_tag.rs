use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "article_tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub article_id: i32,
    pub tag_id: i32,
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Relation {
    Article,
    Tag,
}

impl RelationTrait for Relation {
    fn def(&self) -> sea_orm::RelationDef {
        match self {
            Self::Article => Entity::belongs_to(super::article::Entity)
                .from(Column::ArticleId)
                .to(super::article::Column::Id)
                .into(),
            Self::Tag => Entity::belongs_to(super::tag::Entity)
                .from(Column::TagId)
                .to(super::tag::Column::Id)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
