use serde::Deserialize;

#[derive(Deserialize)]
pub struct CategoryForm {
    pub id: Option<i32>,
    pub name: String,
}
