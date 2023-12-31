//! This module provides a [`Router`] which represent the HTTP endpoints.

use crate::{
    card::InvalidConversion,
    classify::classify,
    deck::draw_hand,
    hand::{Hand, HandCategory, HandConstructionError},
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

/// Creates a router with two endpoints.
///
/// The endpoints:
/// * `/draw`
///    * `GET` generates a hand of five cards, returns a JSON representation of it and a classification of the hand.
///* `/analyze/:cards`
///    * `GET` analyzes the provided cards. The `:cards` format is a comma-separated list of rank and suit for five cards.
///      Example: `/analyze/tr,jr,qr,kr,1r` would return the JSON string "StraightFlush".
#[allow(clippy::doc_markdown)]
pub fn create<B: axum::body::HttpBody + Send + 'static>() -> Router<(), B> {
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
        .map(str::parse)
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
