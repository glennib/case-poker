//! An AWS lambda function runtime

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), lambda_http::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let router = server::router::create();
    lambda_http::run(router).await
}
