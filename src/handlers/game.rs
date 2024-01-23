use crate::{
    models::{game::GameGetById, game_match::GameMatchGetById},
    repositories::{game::GameRepository, game_match::GameMatchRepository, odds::OddsRepository},
    templates::{Game, Match, Menu, MenuItem},
};
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};

use serde::Deserialize;
use uuid::Uuid;

use crate::common::repository::{DbGetByForeignKey, DbReadAll, DbReadMany, DbReadOne};

#[derive(Deserialize)]
pub struct GameId {
    pub game_id: String,
}

pub async fn game_handler(
    Extension(mut game_repository): Extension<GameRepository>,
    Extension(mut game_match_repo): Extension<GameMatchRepository>,
    Extension(mut odds_repo): Extension<OddsRepository>,
    Path(GameId { game_id }): Path<GameId>,
) -> impl IntoResponse {
    let game = game_repository
        .read_one(&GameGetById {
            id: Uuid::parse_str(&game_id.clone()).unwrap(),
        })
        .await
        .unwrap();

    let matches = game_match_repo.get_by_foreign_key(&game.id).await.unwrap();

    let mut matches_to_render = Vec::new();

    for game_match in matches {
        let odds = &odds_repo
            .read_many(&GameMatchGetById { id: game_match.id })
            .await
            .unwrap()[0]; // There always will be exactly one active Odd

        matches_to_render.push(Match {
            match_id: game_match.id,
            team_a: game_match.name_a,
            team_b: game_match.name_b,
            current_odds: odds.to_owned(),
        });
    }

    let template = Game {
        game_name: game.name.clone(),
        matches: matches_to_render,
        game_id: game_id.clone(),
    };

    let menu_items = game_repository
        .read_all()
        .await
        .unwrap()
        .iter()
        .map(|game| MenuItem {
            name: game.name.clone(),
            game_id: game.id,
            active: game.id.clone().to_string() == game_id,
        })
        .collect();

    let menu = Menu { games: menu_items }.render().unwrap();
    let game = template.render().unwrap();

    let response = format!("{menu}{game}");
    (StatusCode::OK, Html(response).into_response())
}
