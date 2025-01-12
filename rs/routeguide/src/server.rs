use std::sync::Arc;

use anyhow::Context;
use futures::stream::BoxStream;
use tokio::{fs::File, io, sync::RwLock};
use tonic::{Request, Response, Status, Streaming};

use crate::route_guide_server::{RouteGuide, RouteGuideServer};

#[derive(Debug, Clone, Default)]
pub struct RouteGuideService {
    features: Arc<RwLock<Vec<crate::Feature>>>,
}

impl RouteGuideService {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self) -> RouteGuideServer<Self> {
        RouteGuideServer::new(self)
    }

    #[tracing::instrument]
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let features = crate::Feature::db_loader()
            .open(path)
            .with_context(|| format!("Failed to open file {path}"))?
            .load()
            .with_context(|| format!("Failed to read file {path}"))?;
        tracing::info!("Read features");
        let features = Arc::new(RwLock::new(features));
        Ok(Self { features })
    }

    pub fn runtime_loader(&self) -> RuntimeLoader<'_> {
        RuntimeLoader {
            service: self,
            reader: (),
        }
    }

    async fn find_feature_at(&self, location: &crate::Point) -> Option<crate::Feature> {
        self.features
            .read()
            .await
            .iter()
            .find(|f| f.location.is_some_and(|ref f| location == f))
            .cloned()
    }

    #[tracing::instrument(skip(self))]
    fn filter_stream_features<'a, 'b>(
        &'a self,
        in_rect: &'b crate::Rectangle,
    ) -> impl futures::Stream<Item = crate::Feature> + Send + use<'a> {
        use futures::{SinkExt, StreamExt};

        let (mut tx, rx) = futures::channel::mpsc::channel(2);
        let in_rect = *in_rect;
        let tx_future = async move {
            let features = self.features.read().await;
            futures::pin_mut!(features);
            let it = features
                .iter()
                .filter(move |f| f.location.as_ref().is_some_and(|l| in_rect.contains(l)));
            for f in it {
                tx.feed(Some(f.clone()))
                    .await
                    .inspect_err(|e| {
                        tracing::error!(
                            error = e as &dyn std::error::Error,
                            "Internal channel error"
                        )
                    })
                    .unwrap();
            }
            tx.close_channel();
            tracing::debug!("Done filtering");
            None
        };
        let tx_stream = futures::stream::once(tx_future);
        futures::stream::select(tx_stream, rx).filter_map(futures::future::ready)
    }

    async fn traverse_points<S, E>(&self, mut points: S) -> Result<crate::RouteSummary, E>
    where
        S: futures::Stream<Item = Result<crate::Point, E>> + Unpin + Send,
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
            if self.find_feature_at(&point).await.is_some() {
                feature_count += 1;
            }
        }
        let end = Instant::now();
        let res = crate::RouteSummary {
            point_count: count,
            feature_count,
            distance: distance as i32,
            elapsed_time: (end - start).as_secs() as i32,
        };
        Ok(res)
    }
}

#[tonic::async_trait]
impl RouteGuide for RouteGuideService {
    #[tracing::instrument(skip(self))]
    async fn get_feature(
        &self,
        request: Request<crate::Point>,
    ) -> Result<Response<crate::Feature>, Status> {
        tracing::debug!("Get features");
        let (_, _, request) = request.into_parts();
        let Some(response) = self.find_feature_at(&request).await else {
            tracing::info!("No feature found");
            return Err(Status::not_found("No feature found"));
        };
        Ok(Response::new(response.clone()))
    }

    type ListFeaturesStream = BoxStream<'static, Result<crate::Feature, Status>>;

    #[tracing::instrument(skip(self))]
    async fn list_features(
        &self,
        request: Request<crate::Rectangle>,
    ) -> Result<Response<Self::ListFeaturesStream>, Status> {
        use futures::StreamExt;

        tracing::debug!("List features");
        let (_, _, request) = request.into_parts();
        let s = self.clone();
        let stream = async_stream::stream! {
            for await f in s.filter_stream_features(&request) {
                yield Ok(f);
            }
        };
        Ok(Response::new(stream.boxed()))
    }

    #[tracing::instrument(skip_all)]
    async fn record_route(
        &self,
        request: Request<Streaming<crate::Point>>,
    ) -> Result<Response<crate::RouteSummary>, Status> {
        tracing::debug!("Record route");
        let (_, _, points) = request.into_parts();
        let summary = self.traverse_points(points).await?;
        tracing::debug!(?summary, "Done recording");
        Ok(Response::new(summary))
    }

    type RouteChatStream = BoxStream<'static, Result<crate::RouteNote, Status>>;

    #[tracing::instrument(skip_all)]
    async fn route_chat(
        &self,
        request: Request<Streaming<crate::RouteNote>>,
    ) -> Result<Response<Self::RouteChatStream>, Status> {
        use futures::{StreamExt, TryStreamExt};
        use std::collections::HashMap;
        use tokio_stream::wrappers::ReceiverStream;

        tracing::debug!("Route chat");
        let (_, _, mut notes) = request.into_parts();
        let (tx, rx) = tokio::sync::mpsc::channel(2);
        let tx_future = async move {
            let mut read_notes: HashMap<_, Vec<_>> = HashMap::new();
            while let Some(note) = notes.try_next().await? {
                let Some(location) = note.location else {
                    continue;
                };
                let matches = read_notes.entry(location).or_default();
                matches.push(note);
                for n in matches {
                    tx.send(Ok(Some(n.clone()))).await.map_err(|e| {
                        tracing::error!(error = &e as &dyn std::error::Error, "Send failed");
                        Status::internal("")
                    })?;
                }
            }
            tracing::debug!("Done route chat");
            Ok(None)
        };
        let tx_stream = futures::stream::once(tx_future);
        let rx_stream = ReceiverStream::new(rx);
        let stream = futures::stream::select(tx_stream, rx_stream)
            .filter_map(|r| async move { r.transpose() });
        Ok(Response::new(stream.boxed()))
    }
}

// MARK: RuntimeLoader

pub struct RuntimeLoader<'a, R = ()> {
    service: &'a RouteGuideService,
    reader: R,
}

impl<'a, R> RuntimeLoader<'a, R> {
    pub async fn open(
        self,
        path: impl AsRef<std::path::Path>,
    ) -> io::Result<RuntimeLoader<'a, File>> {
        let Self { service, reader: _ } = self;
        let file = File::open(path).await?;
        Ok(RuntimeLoader {
            service,
            reader: file,
        })
    }

    pub fn with_reader<R2>(self, reader: R2) -> RuntimeLoader<'a, R2>
    where
        R2: io::AsyncRead + Unpin,
    {
        let Self { service, reader: _ } = self;
        RuntimeLoader { service, reader }
    }

    pub async fn load(self) -> anyhow::Result<()>
    where
        R: io::AsyncRead + Unpin,
    {
        use io::AsyncReadExt;

        let Self {
            service,
            mut reader,
        } = self;
        let buf = {
            let mut buf = String::new();
            reader
                .read_to_string(&mut buf)
                .await
                .context("Failed to read input")?;
            buf
        };
        let mut new_features: Vec<crate::Feature> =
            serde_json::from_str(&buf).context("Failed to parse features JSON")?;
        let mut features = service.features.write().await;
        features.append(&mut new_features);
        Ok(())
    }
}
