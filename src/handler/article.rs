use axum::Extension;
use sea_orm::EntityTrait;
use std::sync::Arc;

use crate::{
    entity::{article, category},
    state::AppState,
    AppError, Result,
};

use super::{get_conn, log_error};
pub async fn index(Extension(state): Extension<Arc<AppState>>) -> Result<String> {
    let handler_name = "article/index";
    let conn = get_conn(&state);
    let (art, cat): (article::Model, Option<category::Model>) = article::Entity::find_by_id(1)
        .find_also_related(category::Entity)
        .one(conn)
        .await
        .map_err(AppError::from)
        .map_err(log_error(handler_name))?
        .ok_or(AppError::notfound())?;
    let cat = cat.ok_or(AppError::notfound())?;
    Ok(format!(
        "文章ID：{}, 文章标题：{}, 分类ID：{}, 分类名称: {}",
        &art.id, &art.title, &cat.id, &cat.name
    ))
}
