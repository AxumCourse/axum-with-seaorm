use axum::routing::get;

use crate::handler;

pub fn init() -> axum::Router {
    axum::Router::new()
        .route("/", get(handler::index))
        .route("/category", get(handler::category::index))
}
