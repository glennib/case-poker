use axum::{
    Json,
    Router,
    routing::get,
};
use serde::Serialize;
use serde_json::Value;
use tracing::{debug, instrument};
use crate::{
    deck::draw_hand,
    hands::{classify, Hand, HandCategory},
};

pub fn create_router() -> Router {
    Router::new().route("/draw", get(draw_and_analyze))
}

#[derive(Serialize)]
struct DrawAndAnalyzeResponse {
    hand: Hand,
    category: HandCategory,
}

#[instrument]
async fn draw_and_analyze() -> Json<Value> {
    debug!("serving");
    let hand = draw_hand();
    let category = classify(&hand);
    Json(serde_json::to_value(DrawAndAnalyzeResponse {
        hand,
        category,
    }).expect("no known fail modes for the Serialize implementations"))
}
