use anyhow::Context;

use routeguide as lib;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use tower::{ServiceBuilder, ServiceExt};
    use tracing_subscriber::EnvFilter;

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into());
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

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
    let db_path =
        std::env::var("ROUTE_GUIDE_DB").unwrap_or_else(|_| "data/route_guide_db.json".to_string());
    let service = lib::server::RouteGuideService::load(&db_path)?.build();
    let trace_layer = tower_http::trace::TraceLayer::new_for_grpc();
    let service = ServiceBuilder::new()
        .layer(trace_layer)
        .service(service)
        .map_request(|r: http::Request<_>| r)
        .map_response(|r: http::Response<_>| r.map(axum::body::Body::new));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("Failed to bind {addr}"))?;
    tracing::info!(%addr, "listening");
    let make_service = axum::ServiceExt::into_make_service(service);
    axum::serve(listener, make_service).await?;

    Ok(())
}
