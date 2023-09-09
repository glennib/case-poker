//! This module provides a [`Router`] which represent the HTTP endpoints.

use crate::{
    cards::{Card, InvalidConversion},
    deck::draw_hand,
    hands::{classify, Hand, HandCategory, HandConstructionError},
};
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use serde_json::Value;
use tracing::{debug, instrument};

pub fn create() -> Router {
    Router::new()
        .route("/draw", get(draw_and_analyze))
        .route("/analyze/:cards", get(analyze))
}

#[derive(Serialize)]
struct DrawAndAnalyzeResponse {
    hand: Hand,
    category: HandCategory,
}

/// Draws a hand of five cards from a deck of 52, returns the hand and its classification.
#[instrument]
async fn draw_and_analyze() -> Json<Value> {
    debug!("serving");
    let hand = draw_hand();
    let category = classify(&hand);
    Json(
        serde_json::to_value(DrawAndAnalyzeResponse { hand, category })
            .expect("no known fail modes"),
    )
}

/// Analyzes the provided hand of five cards and returns its classification. Cards are
/// comma-separated.
///
/// Example request path: /tr,jr,qr,kr,1r
#[instrument]
async fn analyze(Path(cards): Path<String>) -> axum::response::Result<Json<Value>> {
    debug!("serving");

    // Convert each card to a Card. Return error if conversion fails.
    let cards = cards
        .split(',')
        .map(Card::try_from)
        .collect::<Result<Vec<_>, _>>()?;
    // Convert Vec of Card to Hand, return error if conversion fails.
    let hand = Hand::try_from(cards.as_slice())?;
    let category = classify(&hand);
    Ok(Json(
        serde_json::to_value(category).expect("no known fail modes"),
    ))
}

impl IntoResponse for InvalidConversion {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, format!("card is invalid: {self}")).into_response()
    }
}

impl IntoResponse for HandConstructionError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, format!("hand is invalid: {self}")).into_response()
    }
}
