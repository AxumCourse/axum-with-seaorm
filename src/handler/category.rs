use std::sync::Arc;

use axum::{extract::Query, Extension};
use sea_orm::{ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

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
    let mut selc = category::Entity::find().filter(
        Condition::all()
            .add_option(
                params
                    .keyword_opt()
                    .map(|n| category::Column::Name.contains(&n)),
            )
            .add_option(params.is_del_opt().map(|n| category::Column::IsDel.eq(n))),
    );
    if let Some(ord) = params.order() {
        selc = selc.order_by(category::Column::Id, ord);
    };
    let paginator = selc.paginate(conn, page_size);
    let categies: Vec<category::Model> =
        paginator.fetch_page(page).await.map_err(AppError::from)?;
    let tpl = view::CategoryTemplate { categies, params };
    render(tpl, handler_name)
}
