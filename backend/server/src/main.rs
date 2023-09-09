use crate::router::create_router;
use tracing::info;

mod cards;
mod deck;
mod hands;

mod router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let addr = "0.0.0.0:8080".parse().unwrap();
    info!(?addr, "starting server");
    let app = create_router();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
