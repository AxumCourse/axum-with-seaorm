use askama::Template;

use crate::{entity, param};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "category.html")]
pub struct CategoryTemplate {
    pub params: param::CategoryParams,
    pub categies: Vec<entity::category::Model>,
    pub page_total: usize,
}
#[derive(Template)]
#[template(path = "category-add.html")]
pub struct CategoryAddTemplate {}
