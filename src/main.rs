use axum_with_seaorm::{config, router};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "axum_with_seaorm=debug");
    }
    tracing_subscriber::fmt::init();

    dotenv().ok();
    let cfg = config::Config::from_env().unwrap();

    tracing::info!("Web服务监听于{}", &cfg.web.addr);

    let app = router::init();
    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
