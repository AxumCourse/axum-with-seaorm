use axum::{extract::Query, Extension};
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect};
use std::sync::Arc;

use crate::{
    entity::{article, category},
    handler::render,
    param,
    state::AppState,
    view, AppError, Result,
};

use super::{get_conn, log_error, HtmlRespon};
pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    Query(params): Query<param::ArticleParams>,
) -> Result<HtmlRespon> {
    let handler_name = "article/index";
    let conn = get_conn(&state);
    let condition = Condition::all().add(article::Column::IsDel.eq(false));
    let selc = article::Entity::find().filter(condition);
    let record_total = selc
        .clone()
        .count(conn)
        .await
        .map_err(AppError::from)
        .map_err(log_error(handler_name))?;
    let page_size = 15usize;
    let page = 0usize;
    let page_total = f64::ceil(record_total as f64 / page_size as f64) as usize;
    let offset = page_size * page;
    let list = selc
        .find_also_related(category::Entity)
        .limit(page_size as u64)
        .offset(offset as u64)
        .all(conn)
        .await
        .map_err(AppError::from)
        .map_err(log_error(handler_name))?;

    let tpl = view::ArticlesTemplate {
        list,
        page_total,
        params,
    };
    render(tpl, handler_name)
}
