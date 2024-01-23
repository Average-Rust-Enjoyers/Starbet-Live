use crate::{
    models::odds::Odds,
    templates::{Game, Match, Menu, MenuItem, Team},
};
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct GameName {
    name: String,
}

const GAMES: [&str; 4] = ["CS:GO", "Dota 2", "LoL", "Valorant"];

fn mock_matches() -> Vec<Match> {
    vec![
        Match {
            team_a: Team {
                name: "T1".to_string(),
            },
            team_b: Team {
                name: "GenG".to_string(),
            },
            current_odds: Odds {
                id: Uuid::from_u128(420_u128),
                created_at: Utc::now(),
                deleted_at: None,
                game_match_id: Uuid::from_u128(69_u128),
                odds_a: 1.5,
                odds_b: 2.5,
            },
        },
        Match {
            team_a: Team {
                name: "Dplus KIA".to_string(),
            },
            team_b: Team {
                name: "OK BRION".to_string(),
            },
            current_odds: Odds {
                id: Uuid::from_u128(420_u128),
                created_at: Utc::now(),
                deleted_at: None,
                game_match_id: Uuid::from_u128(69_u128),
                odds_a: 2.3,
                odds_b: 1.2,
            },
        },
    ]
}

pub async fn game_handler(Path(GameName { name }): Path<GameName>) -> impl IntoResponse {
    let template = Game {
        game_name: name.clone(),
        matches: mock_matches(),
    };

    let menu_items = GAMES
        .iter()
        .map(|game| MenuItem {
            name: (*game).to_string(),
            active: *game == name.clone(),
        })
        .collect();

    let menu = Menu { games: menu_items }.render().unwrap();
    let game = template.render().unwrap();

    let response = format!("{menu}{game}");
    (StatusCode::OK, Html(response).into_response())
}
