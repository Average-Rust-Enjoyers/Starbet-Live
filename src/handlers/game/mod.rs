use crate::templates::{Game, Menu, MenuItem};
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GameName {
    name: String,
}

const GAMES: [&str; 4] = ["CS:GO", "Dota 2", "LoL", "Valorant"];

pub async fn game_handler(Path(GameName { name }): Path<GameName>) -> impl IntoResponse {
    let template = Game {
        game_name: name.clone(),
        matches: vec![],
    };

    let menu_items = GAMES
        .iter()
        .map(|game| MenuItem {
            name: game.to_string(),
            active: *game == name.clone(),
        })
        .collect();

    let menu = Menu { games: menu_items }.render().unwrap();
    let game = template.render().unwrap();

    let response = format!("{}{}", menu, game);
    (StatusCode::OK, Html(response).into_response())
}
