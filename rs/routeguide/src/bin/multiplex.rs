use anyhow::Context;

use routeguide as lib;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tower_http::trace::TraceLayer;
    use tracing_subscriber::EnvFilter;

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into());
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let db_path =
        std::env::var("ROUTE_GUIDE_DB").unwrap_or_else(|_| "data/route_guide_db.json".to_string());
    let grpc_service = lib::server::RouteGuideService::load(&db_path)?.build();
    let http_router = axum::Router::<()>::new()
        .route("/ping", axum::routing::get(|| async { "pong".to_string() }))
        .layer(TraceLayer::new_for_http());
    let router = tonic::service::Routes::new(grpc_service)
        .into_axum_router()
        .nest("/", http_router)
        .layer(TraceLayer::new_for_grpc());

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|e| {
            tracing::warn!(
                error = &e as &dyn std::error::Error,
                "Failed to load PORT, falling back to default value"
            );
            // `grpc` typed on telephone
            4772.to_string()
        })
        .parse()
        .context("failed to parse PORT value")?;
    let addr: std::net::SocketAddr = ([0, 0, 0, 0], port).into();

    tracing::info!(%addr, "listening");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("Failed to bind {addr}"))?;
    axum::serve(listener, router).await?;

    Ok(())
}
