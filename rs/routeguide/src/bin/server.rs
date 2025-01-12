use anyhow::Context;

use routeguide as lib;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
    let service = lib::RouteGuideService::load(&db_path)?;
    let trace_layer = tower_http::trace::TraceLayer::new_for_grpc();
    let server = tonic::transport::Server::builder()
        .layer(trace_layer)
        .add_service(service.build());
    tracing::info!(%addr, "listening");
    server.serve(addr).await?;

    Ok(())
}
