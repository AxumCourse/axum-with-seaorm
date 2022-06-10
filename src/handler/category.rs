use std::sync::Arc;

use axum::{
    extract::{Form, Path, Query},
    Extension,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    Set,
};

use super::{get_conn, log_error, redirect, render, HtmlRespon, RedirectRespon};
use crate::{entity::category, form, param, state::AppState, view, AppError, Result};

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
    let page_total = paginator.num_pages().await.map_err(AppError::from)?;
    let categies: Vec<category::Model> = paginator
        .fetch_page(page)
        .await
        .map_err(AppError::from)
        .map_err(log_error(handler_name))?;
    let tpl = view::CategoryTemplate {
        categies,
        params,
        page_total,
    };
    render(tpl, handler_name)
}

pub async fn find(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<String> {
    let handler_name = "category/find";
    let conn = get_conn(&state);
    let cate: Option<category::Model> = category::Entity::find_by_id(id)
        .one(conn)
        .await
        .map_err(AppError::from)
        .map_err(log_error(handler_name))?;
    match cate {
        Some(cate) => Ok(format!("id: {}, 名称: {}", cate.id, cate.name)),
        None => Err(AppError::notfound()),
    }
}

pub async fn add_ui() -> Result<HtmlRespon> {
    let handler_name = "category/add_ui";
    let tpl = view::CategoryAddTemplate {};
    render(tpl, handler_name)
}
pub async fn add(
    Extension(state): Extension<Arc<AppState>>,
    Form(frm): Form<form::CategoryForm>,
) -> Result<RedirectRespon> {
    let handler_name = "category/add";
    let conn = get_conn(&state);
    let am = category::ActiveModel {
        name: Set(frm.name),
        ..Default::default()
    };
    let added_category: category::Model = am
        .insert(conn)
        .await
        .map_err(AppError::from)
        .map_err(log_error(handler_name))?;
    let url = format!("/category?msg=分类添加成功，ID是：{}", added_category.id);
    redirect(url.as_str())
}
