use serde::Deserialize;

#[derive(Deserialize)]
pub struct CategoryForm {
    pub name: String,
}
#[derive(Deserialize)]
pub struct ArticleForm {
    pub title: String,
    pub category_id: i32,
    pub content: String,
}
