mod card;
mod classify;
mod deck;
mod hand;
mod router;

use router::create;
use tracing::info;

/// Start a web server at port 8080.
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let addr = "0.0.0.0:8080".parse().unwrap();
    info!(?addr, "starting server");
    let router = create();
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
