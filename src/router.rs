use axum::routing::get;

use crate::handler;

pub fn init() -> axum::Router {
    axum::Router::new()
        .route("/", get(handler::index))
        .route("/category", get(handler::category::index))
        .route(
            "/category/add",
            get(handler::category::add_ui).post(handler::category::add),
        )
        .route(
            "/category/edit/:id",
            get(handler::category::edit_ui).post(handler::category::edit),
        )
        .route("/category/del/:id", get(handler::category::del))
        .route("/category/del/:id/:real", get(handler::category::del))
        .route("/category/articles/:id", get(handler::category::articles))
        .route("/article", get(handler::article::index))
        .route(
            "/article/add",
            get(handler::article::add_ui).post(handler::article::add),
        )
}
