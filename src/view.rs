use askama::Template;

use crate::entity;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "category.html")]
pub struct CategoryTemplate {
    pub categies: Vec<entity::category::Model>,
}
