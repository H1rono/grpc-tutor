use anyhow::Context;

use helloworld as lib;

type Request = tonic::Request<lib::HelloRequest>;

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
    tracing::info!(%endpoint, "Connect client");
    let mut client = lib::client::GreeterClient::connect(endpoint)
        .await
        .context("Failed to connect")?;
    let request = Request::new(lib::HelloRequest {
        name: "Tonic".to_string(),
    });
    let response = client.say_hello(request).await.context("Failed to call")?;
    tracing::info!(?response, "Received response");

    Ok(())
}
