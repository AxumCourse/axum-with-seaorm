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

#[derive(Template)]
#[template(path = "category-edit.html")]
pub struct CategoryEditTemplate {
    pub category: entity::category::Model,
}
#[derive(Template)]
#[template(path = "category-articles.html")]
pub struct CategoryArticlesTemplate {
    pub params: param::CategoryParams,
    pub page_total: usize,
    pub category: entity::category::Model,
    pub articles: Vec<entity::article::Model>,
}
#[derive(Template)]
#[template(path = "article.html")]
pub struct ArticlesTemplate {
    pub page_total: usize,
    pub list: Vec<(entity::article::Model, Option<entity::category::Model>)>,
    pub params: param::ArticleParams,
}
#[derive(Template)]
#[template(path = "article-add.html")]
pub struct ArticleAddTemplate {
    pub categies: Vec<entity::category::Model>,
}
