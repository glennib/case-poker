use tracing::info;
use crate::router::create_router;

mod cards;
mod hands;
mod deck;

mod router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let addr = "0.0.0.0:8080".parse().unwrap();
    info!(?addr, "starting server");
    let app = create_router();
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
