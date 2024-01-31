use crate::{
    models::{
        extension_web_socket::ExtensionWebSocket,
        game_match::GameMatchGetById,
        odds::{Odds, OddsCreate, OddsGetById},
    },
    repositories::{game_match::GameMatchRepository, odds::OddsRepository},
    templates::{Match, PlaceBetForm},
};

use crate::common::repository::{DbCreate, DbDelete, DbReadMany, DbReadOne};
use askama::Template;
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct BetAmount {
    pub bet_amount: String,
}

pub async fn place_bet_handler(
    Extension(web_socket): Extension<ExtensionWebSocket>,
    Extension(odds_repo): Extension<OddsRepository>,
    Extension(mut match_repo): Extension<GameMatchRepository>,
    Path((match_id, prediction)): Path<(String, String)>,
    Json(bet_amount): Json<BetAmount>,
) -> impl IntoResponse {
    let bet_amount = bet_amount.bet_amount.parse::<i32>().unwrap();
    let new_odds = create_new_odds(
        match_id.clone(),
        prediction.clone(),
        bet_amount,
        odds_repo.clone(),
    )
    .await;

    let game_match = match_repo
        .read_one(&GameMatchGetById {
            id: Uuid::parse_str(&match_id).unwrap(),
        })
        .await
        .unwrap();

    let updated_match_template = Match {
        match_id: new_odds.game_match_id,
        team_a: game_match.name_a,
        team_b: game_match.name_b,
        current_odds: new_odds,
    }
    .render()
    .unwrap();

    web_socket
        .tx
        .send_async(updated_match_template)
        .await
        .unwrap();

    StatusCode::OK
}

pub async fn get_bet_handler(
    Extension(mut game_match_repo): Extension<GameMatchRepository>,
    Path((match_id, prediction)): Path<(String, String)>,
) -> impl IntoResponse {
    let match_id = Uuid::parse_str(&match_id).unwrap();
    let game_match = game_match_repo
        .read_one(&GameMatchGetById { id: match_id })
        .await
        .unwrap();

    let predicted_team = match prediction.as_str() {
        "a" => game_match.name_a,
        _ => game_match.name_b,
    };

    let template = PlaceBetForm {
        match_id: game_match.id,
        predicted_team,
        prediction: prediction.to_string(),
    }
    .render()
    .unwrap();

    (StatusCode::OK, Html(template).into_response())
}

async fn create_new_odds(
    match_id: String,
    prediction: String,
    _bet_amount: i32,
    mut odds_repository: OddsRepository,
) -> Odds {
    let match_uuid = Uuid::parse_str(&match_id).unwrap();
    let current_odds = &odds_repository
        .read_many(&GameMatchGetById { id: match_uuid })
        .await
        .unwrap()[0];

    // mocking updating odds
    let (mut new_odds_a, mut new_odds_b) = match prediction.as_str() {
        "a" => (current_odds.odds_a - 0.1, current_odds.odds_b + 0.1),
        _ => (current_odds.odds_a + 0.1, current_odds.odds_b - 0.1),
    };

    // can I make the current_odds in a numeric format: x.xxx?
    new_odds_a = format!("{:.3}", new_odds_a).parse::<f64>().unwrap();
    new_odds_b = format!("{:.3}", new_odds_b).parse::<f64>().unwrap();

    odds_repository
        .delete(&OddsGetById {
            id: current_odds.id,
        })
        .await
        .unwrap();

    odds_repository
        .create(&OddsCreate {
            game_match_id: match_uuid,
            odds_a: new_odds_a,
            odds_b: new_odds_b,
        })
        .await
        .unwrap()
}
