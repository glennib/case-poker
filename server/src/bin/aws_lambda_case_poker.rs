//! An AWS lambda function runtime

use tracing::info;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), lambda_http::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    info!("starting");

    let router = server::router::create();
    lambda_http::run(router).await
}
