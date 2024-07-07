mod router;
use archery_tools_server::{db, env::ENV};
use router::build_router;

#[tokio::main]
async fn main() {
    // initialize tracing
    std::env::set_var("RUST_LOG", "trace");
    tracing_subscriber::fmt::init();
    // Init logger with default log level of "info"
    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    // Init env because it lazy loads on first use
    let _ = *ENV;
    db::migrate().await.unwrap_or_else(|err| {
        log::error!("DB migration failed: {}", err);
    });

    let router = build_router();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(ENV.server.clone())
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}
