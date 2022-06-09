use std::sync::Arc;

use axum::Extension;
use axum_with_seaorm::{config, router, state, };
use dotenv::dotenv;
use sea_orm::Database;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "axum_with_seaorm=debug");
    }
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let cfg = config::Config::from_env().unwrap();
    let conn = Database::connect(&cfg.db.dsn).await.unwrap();

    tracing::info!("Web服务监听于{}", &cfg.web.addr);

    let app = router::init().layer(Extension(Arc::new(state::AppState { conn })));
    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
