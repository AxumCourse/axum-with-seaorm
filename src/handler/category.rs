use std::sync::Arc;

use axum::{extract::Query, Extension};
use sea_orm::{EntityTrait, PaginatorTrait, QueryOrder};

use super::{get_conn, render, HtmlRespon};
use crate::{entity::category, param, state::AppState, view, AppError, Result};

pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    Query(params): Query<param::CategoryParams>,
) -> Result<HtmlRespon> {
    let handler_name = "category/index";
    let page = params.page(); // 当前页码
    let page_size = params.page_size(); // 每页条数，默认15
    let conn = get_conn(&state);
    let paginator = category::Entity::find().paginate(conn, page_size);
    let categies: Vec<category::Model> =
        paginator.fetch_page(page).await.map_err(AppError::from)?;
    let tpl = view::CategoryTemplate { categies, params };
    render(tpl, handler_name)
}
