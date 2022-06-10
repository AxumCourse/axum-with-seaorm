#[derive(Debug)]
pub enum AppErrorType {
    Database,
    Template,
    Notfound,
}

type Cause = Box<dyn std::error::Error>;

#[derive(Debug)]
pub enum AppErrorItem {
    Message(String),
    Cause(Cause),
}

#[derive(Debug)]
pub struct AppError {
    pub types: AppErrorType,
    pub error: AppErrorItem,
}

impl AppError {
    pub fn new(types: AppErrorType, error: AppErrorItem) -> Self {
        Self { types, error }
    }
    pub fn from_err(cause: Cause, types: AppErrorType) -> Self {
        Self::new(types, AppErrorItem::Cause(cause))
    }
    pub fn from_msg(msg: &str, types: AppErrorType) -> Self {
        Self::new(types, AppErrorItem::Message(msg.to_string()))
    }
    pub fn notfound() -> Self {
        Self::from_msg("不存在的记录", AppErrorType::Notfound)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AppError {}

impl From<askama::Error> for AppError {
    fn from(err: askama::Error) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Template)
    }
}
impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        Self::from_err(Box::new(err), AppErrorType::Database)
    }
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let msg = match self.error {
            AppErrorItem::Cause(err) => err.to_string(),
            AppErrorItem::Message(msg) => msg.to_string(),
        };
        msg.into_response()
    }
}
