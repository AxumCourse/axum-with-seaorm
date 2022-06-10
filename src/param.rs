use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CategoryParams {
    pub keyword: Option<String>,
    pub is_del: Option<i32>,
    pub sort: Option<String>,
    pub page_size: Option<i32>,
}

impl CategoryParams {
    pub fn keyword(&self) -> String {
        self.keyword.clone().unwrap_or("".to_string())
    }
    pub fn is_del(&self) -> i32 {
        self.is_del.unwrap_or(-1)
    }
    pub fn sort(&self) -> String {
        self.sort.clone().unwrap_or("".to_string())
    }
    pub fn page_size(&self) -> i32 {
        self.page_size.unwrap_or(15)
    }
}

impl Default for CategoryParams {
    fn default() -> Self {
        Self {
            keyword: None,
            is_del: None,
            sort: None,
            page_size: None,
        }
    }
}
