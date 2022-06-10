use std::sync::Arc;

use axum::{extract::Query, Extension};
use sea_orm::{EntityTrait, QueryOrder};

use super::{get_conn, render, HtmlRespon};
use crate::{entity::category, param, state::AppState, view, AppError, Result};

pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    Query(params): Query<param::CategoryParams>,
) -> Result<HtmlRespon> {
    tracing::debug!("接收到的URL参数：{:?}", params);
    let handler_name = "category/index";
    let conn = get_conn(&state);
    let categies: Vec<category::Model> = category::Entity::find()
        .order_by_asc(category::Column::Id)
        .all(conn)
        .await
        .map_err(AppError::from)?;
    let tpl = view::CategoryTemplate { categies, params };
    render(tpl, handler_name)
}
