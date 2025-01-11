use anyhow::Context;
use futures::stream::BoxStream;
use tonic::{Request, Response, Status, Streaming};

use routeguide as lib;

#[derive(Debug, Clone)]
struct RouteGuideService;

#[tonic::async_trait]
impl lib::server::RouteGuide for RouteGuideService {
    async fn get_feature(
        &self,
        request: Request<lib::Point>,
    ) -> Result<Response<lib::Feature>, Status> {
        todo!()
    }

    type ListFeaturesStream = BoxStream<'static, Result<lib::Feature, Status>>;

    async fn list_features(
        &self,
        request: Request<lib::Rectangle>,
    ) -> Result<Response<Self::ListFeaturesStream>, Status> {
        todo!()
    }

    async fn record_route(
        &self,
        request: Request<Streaming<lib::Point>>,
    ) -> Result<Response<lib::RouteSummary>, Status> {
        todo!()
    }

    type RouteChatStream = BoxStream<'static, Result<lib::RouteNote, Status>>;

    async fn route_chat(
        &self,
        request: Request<Streaming<lib::RouteNote>>,
    ) -> Result<Response<Self::RouteChatStream>, Status> {
        todo!()
    }
}

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
    let service = RouteGuideService;
    let trace_layer = tower_http::trace::TraceLayer::new_for_grpc();
    let server = tonic::transport::Server::builder()
        .layer(trace_layer)
        .add_service(lib::server::RouteGuideServer::new(service));
    tracing::info!(%addr, "listening");
    server.serve(addr).await?;

    Ok(())
}
