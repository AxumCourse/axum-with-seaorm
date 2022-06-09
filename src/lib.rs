pub mod config;
mod err;
pub mod handler;
pub mod router;
pub mod view;
pub mod entity;

pub use err::{AppError, AppErrorType};

pub type Result<T> = std::result::Result<T, crate::AppError>;
