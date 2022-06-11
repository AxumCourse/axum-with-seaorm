use serde::Deserialize;

const DEFAULT_PAGESIZE: usize = 15;
#[derive(Debug, Clone, Deserialize)]
pub struct CategoryParams {
    pub keyword: Option<String>,
    pub is_del: Option<i32>,
    pub sort: Option<String>,
    pub page_size: Option<usize>,
    pub page: Option<usize>,
    pub msg: Option<String>,
}

impl CategoryParams {
    pub fn keyword(&self) -> String {
        self.keyword.clone().unwrap_or("".to_string())
    }
    pub fn keyword_opt(&self) -> Option<String> {
        match &self.keyword {
            Some(s) => {
                if s.is_empty() {
                    None
                } else {
                    Some(s.to_string())
                }
            }
            _ => None,
        }
    }
    pub fn is_del_opt(&self) -> Option<bool> {
        match self.is_del {
            Some(n) => match n {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            },
            _ => None,
        }
    }
    pub fn is_del(&self) -> i32 {
        match self.is_del_opt() {
            Some(b) => {
                if b {
                    1
                } else {
                    0
                }
            }
            _ => -1,
        }
    }
    pub fn sort(&self) -> String {
        self.sort.clone().unwrap_or("".to_string())
    }
    pub fn order(&self) -> Option<sea_orm::Order> {
        match self.sort().as_str() {
            "asc" => Some(sea_orm::Order::Asc),
            "desc" => Some(sea_orm::Order::Desc),
            _ => None,
        }
    }
    pub fn page_size(&self) -> usize {
        let ps = self.page_size.unwrap_or(0);
        if ps <= 0 {
            return DEFAULT_PAGESIZE;
        }
        ps
    }
    pub fn page(&self) -> usize {
        self.page.unwrap_or(0)
    }
}

impl Default for CategoryParams {
    fn default() -> Self {
        Self {
            keyword: None,
            is_del: None,
            sort: None,
            page_size: None,
            page: None,
            msg: None,
        }
    }
}
#[derive(Deserialize, Debug)]
pub struct DelParams {
    pub id: i32,
    pub real: Option<bool>,
}

pub type ArticleParams = CategoryParams;
