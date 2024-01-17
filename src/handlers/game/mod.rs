use crate::templates::Game;
use askama::Template;
use axum::{
    extract::Json,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GameInput {
    game_name: String,
}

pub async fn game_handler(Json(input): Json<GameInput>) -> impl IntoResponse {
    let template = Game {
        game_name: input.game_name,
        matches: vec![],
    };

    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
