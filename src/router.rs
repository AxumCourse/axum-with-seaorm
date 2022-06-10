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
}
