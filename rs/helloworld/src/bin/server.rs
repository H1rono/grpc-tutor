use anyhow::Context;

use helloworld as lib;

type Request = tonic::Request<lib::HelloRequest>;
type Response = tonic::Response<lib::HelloReply>;

#[derive(Debug, Clone, Copy, Default)]
struct MyGreeter;

#[tonic::async_trait]
impl lib::server::Greeter for MyGreeter {
    #[tracing::instrument(skip(self))]
    async fn say_hello(&self, request: Request) -> Result<Response, tonic::Status> {
        tracing::info!("Got a request");
        let (_, _, request) = request.into_parts();
        let rep_message = format!("Hello, {}!", request.name);
        let reply = lib::HelloReply {
            message: rep_message,
        };
        Ok(Response::new(reply))
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
    let greeter = MyGreeter;
    let trace_layer = tower_http::trace::TraceLayer::new_for_grpc();
    let server = tonic::transport::Server::builder()
        .layer(trace_layer)
        .add_service(lib::server::GreeterServer::new(greeter));
    tracing::info!(%addr, "listening");
    server.serve(addr).await?;

    Ok(())
}
