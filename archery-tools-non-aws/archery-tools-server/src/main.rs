mod router;
use archery_tools_server::env::ENV;
use router::build_router;

#[tokio::main]
async fn main() {
    // Init logger with default log level of "info"
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    // Init env because it lazy loads on first use
    let _ = *ENV;
    // initialize tracing
    // tracing_subscriber::fmt::init();

    let router = build_router();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(ENV.server.clone())
        .await
        .unwrap();
    axum::serve(listener, router).await.unwrap();
}
