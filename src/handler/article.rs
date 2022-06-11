use axum::{
    extract::{Form, Query},
    Extension,
};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use std::sync::Arc;

use crate::{
    entity::{article, category, tag},
    form,
    handler::render,
    param,
    state::AppState,
    view, AppError, Result,
};

use super::{get_conn, log_error, redirect, HtmlRespon, RedirectRespon};
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
        .order_by_desc(article::Column::Id)
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
pub async fn add_ui(Extension(state): Extension<Arc<AppState>>) -> Result<HtmlRespon> {
    let handler_name = "article/add_ui";
    let conn = get_conn(&state);
    let categies = category::Entity::find()
        .filter(category::Column::IsDel.eq(false))
        .limit(100)
        .order_by_asc(category::Column::Id)
        .all(conn)
        .await
        .map_err(AppError::from)
        .map_err(log_error(handler_name))?;
    let tpl = view::ArticleAddTemplate { categies };
    render(tpl, handler_name)
}
pub async fn add(
    Extension(state): Extension<Arc<AppState>>,
    Form(frm): Form<form::ArticleForm>,
) -> Result<RedirectRespon> {
    let handler_name = "article/add";
    let conn = get_conn(&state);
    article::ActiveModel {
        id: NotSet,
        title: Set(frm.title),
        category_id: Set(frm.category_id),
        content: Set(frm.content),
        ..Default::default()
    }
    .save(conn)
    .await
    .map_err(AppError::from)
    .map_err(log_error(handler_name))?;
    redirect("/article?msg=文章添加成功")
}
pub async fn list_with_tags(Extension(state): Extension<Arc<AppState>>) -> Result<String> {
    let handler_name = "article/list_with_tags";
    let conn = get_conn(&state);
    let list: Vec<(article::Model, Vec<tag::Model>)> = article::Entity::find()
        .find_with_related(tag::Entity)
        .all(conn)
        .await
        .map_err(AppError::from)
        .map_err(log_error(handler_name))?;
    let mut ss = vec![];
    for item in list {
        let (article, tags) = item;
        let tags = tags
            .iter()
            .map(|tag| format!("【#{} - {}】", &tag.id, &tag.name))
            .collect::<Vec<String>>()
            .join(", ")
            .to_string();

        let s = format!(
            "文章ID: {}, 文章标题: {}, 标签： {}",
            &article.id, &article.title, tags,
        );
        ss.push(s);
    }
    Ok(ss.join("\n").to_string())
}
