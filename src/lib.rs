pub mod config;
pub mod entity;
mod err;
pub mod form;
pub mod handler;
pub mod param;
pub mod router;
pub mod state;
pub mod view;

pub use err::{AppError, AppErrorType};

pub type Result<T> = std::result::Result<T, crate::AppError>;
