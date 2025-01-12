use std::sync::Arc;

use anyhow::Context;
use futures::stream::BoxStream;
use tonic::{Request, Response, Status, Streaming};

use routeguide as lib;

#[derive(Debug, Clone)]
struct RouteGuideService {
    features: Arc<Vec<lib::Feature>>,
}

impl RouteGuideService {
    #[tracing::instrument]
    fn load(path: &str) -> anyhow::Result<Self> {
        let features = lib::Feature::db_loader()
            .open(path)
            .with_context(|| format!("Failed to open file {path}"))?
            .load()
            .with_context(|| format!("Failed to read file {path}"))?;
        tracing::info!("Read features");
        let features = Arc::new(features);
        Ok(Self { features })
    }

    fn find_feature_at(&self, location: &lib::Point) -> Option<&lib::Feature> {
        self.features
            .iter()
            .find(|f| f.location.is_some_and(|ref f| location == f))
    }

    fn filter_stream_features<'a, 'b>(
        &'a self,
        in_rect: &'b lib::Rectangle,
    ) -> impl Iterator<Item = &'a lib::Feature> + use<'a, 'b> {
        self.features
            .iter()
            .filter(move |f| f.location.as_ref().is_some_and(|l| in_rect.contains(l)))
    }

    async fn traverse_points<S, E>(&self, mut points: S) -> Result<lib::RouteSummary, E>
    where
        S: futures::Stream<Item = Result<lib::Point, E>> + Unpin + Send,
        E: Send + Sync + 'static,
    {
        use futures::TryStreamExt;
        use std::time::Instant;

        let mut count = 0;
        let mut feature_count = 0;
        let mut last_point = None;
        let mut distance = 0.0;
        let start = Instant::now();
        while let Some(point) = points.try_next().await? {
            count += 1;
            if let Some(lp) = last_point.replace(point) {
                distance += lp.distance_between(&point);
            }
            if self.find_feature_at(&point).is_some() {
                feature_count += 1;
            }
        }
        let end = Instant::now();
        let res = lib::RouteSummary {
            point_count: count,
            feature_count,
            distance: distance as i32,
            elapsed_time: (end - start).as_secs() as i32,
        };
        Ok(res)
    }
}

#[tonic::async_trait]
impl lib::server::RouteGuide for RouteGuideService {
    #[tracing::instrument(skip(self))]
    async fn get_feature(
        &self,
        request: Request<lib::Point>,
    ) -> Result<Response<lib::Feature>, Status> {
        tracing::debug!("Get features");
        let (_, _, request) = request.into_parts();
        let Some(response) = self.find_feature_at(&request) else {
            tracing::info!("No feature found");
            return Err(Status::not_found("No feature found"));
        };
        Ok(Response::new(response.clone()))
    }

    type ListFeaturesStream = BoxStream<'static, Result<lib::Feature, Status>>;

    #[tracing::instrument(skip(self))]
    async fn list_features(
        &self,
        request: Request<lib::Rectangle>,
    ) -> Result<Response<Self::ListFeaturesStream>, Status> {
        use futures::StreamExt;

        tracing::debug!("List features");
        let (_, _, request) = request.into_parts();
        let s = self.clone();
        let stream = async_stream::stream! {
            for f in s.filter_stream_features(&request) {
                yield Ok(f.clone())
            }
            tracing::debug!("Done listing");
        };
        Ok(Response::new(stream.boxed()))
    }

    #[tracing::instrument(skip_all)]
    async fn record_route(
        &self,
        request: Request<Streaming<lib::Point>>,
    ) -> Result<Response<lib::RouteSummary>, Status> {
        tracing::debug!("Record route");
        let (_, _, points) = request.into_parts();
        let summary = self.traverse_points(points).await?;
        tracing::debug!(?summary, "Done recording");
        Ok(Response::new(summary))
    }

    type RouteChatStream = BoxStream<'static, Result<lib::RouteNote, Status>>;

    async fn route_chat(
        &self,
        request: Request<Streaming<lib::RouteNote>>,
    ) -> Result<Response<Self::RouteChatStream>, Status> {
        use futures::{StreamExt, TryStreamExt};
        use std::collections::HashMap;

        tracing::debug!("Route chat");
        let (_, _, mut notes) = request.into_parts();
        let stream = async_stream::try_stream! {
            let mut read_notes: HashMap<_, Vec<_>> = HashMap::new();
            while let Some(note) = notes.try_next().await? {
                let Some(location) = &note.location else {
                    continue;
                };
                let matches = read_notes.entry(*location).or_default();
                matches.push(note);
                for n in matches {
                    yield n.clone()
                }
            }
            tracing::debug!("Done route chat");
        };
        Ok(Response::new(stream.boxed()))
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
    let db_path =
        std::env::var("ROUTE_GUIDE_DB").unwrap_or_else(|_| "data/route_guide_db.json".to_string());
    let service = RouteGuideService::load(&db_path)?;
    let trace_layer = tower_http::trace::TraceLayer::new_for_grpc();
    let server = tonic::transport::Server::builder()
        .layer(trace_layer)
        .add_service(lib::server::RouteGuideServer::new(service));
    tracing::info!(%addr, "listening");
    server.serve(addr).await?;

    Ok(())
}
