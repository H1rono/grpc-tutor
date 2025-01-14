use anyhow::Context;

use routeguide as lib;

type Client = lib::route_guide_client::RouteGuideClient<tonic::transport::Channel>;

#[tracing::instrument(skip_all)]
async fn get_feature(client: &mut Client) -> anyhow::Result<()> {
    let point = lib::Point {
        latitude: 356810420,
        longitude: 1397672140,
    };
    let response = client
        .get_feature(point)
        .await
        .inspect_err(|status| tracing::error!(code = %status.code(), message = %status.message()))
        .context("Server responded with error")?;
    let (_, feature, _) = response.into_parts();
    tracing::info!(name = feature.name, "Received a feature");
    Ok(())
}

#[tracing::instrument(skip_all)]
async fn list_features(client: &mut Client) -> anyhow::Result<()> {
    use futures::StreamExt;

    let lo = lib::Point {
        latitude: 350000000,
        longitude: 1390000000,
    };
    let hi = lib::Point {
        latitude: 360000000,
        longitude: 1400000000,
    };
    let rect = lib::Rectangle {
        lo: Some(lo),
        hi: Some(hi),
    };
    let response = client
        .list_features(rect)
        .await
        .inspect_err(|status| tracing::error!(code = %status.code(), message = %status.message()))
        .context("Server responded with error")?;
    let (_, mut features, _) = response.into_parts();
    while let Some(feature) = features.next().await {
        let feature = feature
            .inspect_err(
                |status| tracing::error!(code = %status.code(), message = %status.message()),
            )
            .context("Server yielded error status")?;
        tracing::info!(name = feature.name, "Received a feature");
    }
    Ok(())
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
    let endpoint = format!("http://localhost:{port}");
    tracing::info!(%endpoint, "Connecting");
    let mut client = lib::route_guide_client::RouteGuideClient::connect(endpoint)
        .await
        .context("Failed to connect")?;
    get_feature(&mut client).await?;
    list_features(&mut client).await?;

    Ok(())
}
