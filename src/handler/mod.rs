use askama::Template;
use axum::response::Html;
use sea_orm::DatabaseConnection;

use crate::{state::AppState, AppError, Result};

mod index;

pub mod category;

pub use index::index;

type HtmlRespon = Html<String>;

/// 渲染模板
fn render<T: Template>(tpl: T, handler_name: &str) -> Result<HtmlRespon> {
    let out = tpl
        .render()
        .map_err(AppError::from)
        .map_err(log_error(handler_name))?;
    Ok(Html(out))
}

/// 记录错误
fn log_error(handler_name: &str) -> Box<dyn Fn(AppError) -> AppError> {
    let handler_name = handler_name.to_string();
    Box::new(move |err| {
        tracing::error!("{}: {:?}", handler_name, err);
        err
    })
}

fn get_conn<'a>(state: &'a AppState) -> &'a DatabaseConnection {
    &state.conn
}
