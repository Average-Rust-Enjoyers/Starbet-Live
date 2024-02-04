use crate::{
    auth::{self, AuthSession},
    common::helpers::{
        format_date_time_string_with_seconds, generate_error_message_template, show_popup_error,
    },
    error::{AppError, AppResult},
    models::{
        bet::{BetCreate, BetGetByUserId},
        extension_web_socket::{ExtensionWebSocketError, ExtensionWebSocketMatch},
        game::GameGetById,
        game_match::{GameMatchGetById, GameMatchStatus},
        game_match_outcome::GameMatchOutcome,
        odds::{Odds, OddsCreate, OddsGetByGameMatchId},
    },
    repositories::{
        bet::BetRepository, game::GameRepository, game_match::GameMatchRepository,
        odds::OddsRepository,
    },
    templates::{ActiveBets, Bet, Match, PlaceBetForm},
};

use crate::common::repository::{DbCreate, DbGetLatest, DbReadMany, DbReadOne};
use askama::Template;
use axum::{
    extract::{Json, Path},
    http::StatusCode,
    response::Html,
    Extension,
};

use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct BetAmount {
    pub bet_amount: String,
}

#[allow(clippy::too_many_arguments)]
pub async fn place_bet_handler(
    auth_session: AuthSession,
    Extension(web_socket): Extension<ExtensionWebSocketMatch>,
    Extension(error_web_socket): Extension<ExtensionWebSocketError>,
    Extension(odds_repo): Extension<OddsRepository>,
    Extension(mut match_repo): Extension<GameMatchRepository>,
    Extension(mut bet_repo): Extension<BetRepository>,
    Extension(game_repo): Extension<GameRepository>,
    Path((match_id, prediction)): Path<(String, String)>,
    Json(bet_amount): Json<BetAmount>,
) -> AppResult<Html<String>> {
    let bet_amount = bet_amount.bet_amount.parse::<i32>()?;

    let user = auth::is_logged_in(auth_session)?;

    if user.balance < bet_amount || bet_amount < 1 {
        let _ = error_web_socket
            .tx
            .send_async(generate_error_message_template(
                "Insufficient funds",
                user.id,
            ))
            .await;

        return Err(AppError::StatusCode(StatusCode::PRECONDITION_FAILED));
    }

    let new_odds = create_new_odds(
        match_id.clone(),
        prediction.clone(),
        bet_amount,
        odds_repo.clone(),
    )
    .await
    .or_else(|e| {
        show_popup_error(
            "Failed to create new odds",
            e,
            user.id,
            error_web_socket.clone(),
        )
    })?;

    let game_match = match_repo
        .read_one(&GameMatchGetById {
            id: Uuid::parse_str(&match_id)?,
        })
        .await
        .or_else(|e| show_popup_error("Match not found", e, user.id, error_web_socket.clone()))?;

    let updated_match_template = Match {
        match_id: new_odds.game_match_id,
        team_a: game_match.name_a,
        team_b: game_match.name_b,
        current_odds: new_odds.clone(),
    }
    .render()?;

    web_socket.tx.send_async(updated_match_template).await?; // TODO: ignore error or not?

    bet_repo
        .create(&BetCreate {
            id: Uuid::new_v4(),
            app_user_id: user.id,
            game_match_id: game_match.id,
            expected_outcome: match prediction.as_str() {
                "a" => GameMatchOutcome::WinA,
                _ => GameMatchOutcome::WinB,
            },
            amount: bet_amount,
            odds_id: new_odds.id,
        })
        .await
        .or_else(|e| {
            show_popup_error("Failed to place bet", e, user.id, error_web_socket.clone())
        })?;

    let bets = get_active_bets_by_user_id(
        bet_repo.clone(),
        match_repo.clone(),
        game_repo.clone(),
        user.id,
    )
    .await
    .or_else(|e| {
        show_popup_error(
            "Failed to get active bets",
            e,
            user.id,
            error_web_socket.clone(),
        )
    })?;

    let bets_history_template = ActiveBets { bets }.render()?;

    Ok(Html(bets_history_template))
}

pub async fn get_bet_handler(
    Extension(mut game_match_repo): Extension<GameMatchRepository>,
    Path((match_id, prediction)): Path<(String, String)>,
) -> AppResult<Html<String>> {
    let match_id = Uuid::parse_str(&match_id)?;
    let game_match = game_match_repo
        .read_one(&GameMatchGetById { id: match_id })
        .await?;

    let predicted_team = match prediction.as_str() {
        "a" => game_match.name_a,
        _ => game_match.name_b,
    };

    let template = PlaceBetForm {
        match_id: game_match.id,
        predicted_team,
        prediction: prediction.to_string(),
    }
    .render()?;

    Ok(Html(template))
}

async fn create_new_odds(
    match_id: String,
    prediction: String,
    _bet_amount: i32,
    mut odds_repository: OddsRepository,
) -> AppResult<Odds> {
    let match_uuid = Uuid::parse_str(&match_id)?;

    let current_odds = odds_repository
        .get_latest(&OddsGetByGameMatchId {
            game_match_id: match_uuid,
        })
        .await?;

    // mocking updating odds
    let (mut new_odds_a, mut new_odds_b) = match prediction.as_str() {
        "a" => (
            (current_odds.odds_a - 0.1).max(1.1),
            (current_odds.odds_b + 0.1).min(10.0),
        ),
        _ => (
            (current_odds.odds_a + 0.1).min(10.0),
            (current_odds.odds_b - 0.1).max(1.1),
        ),
    };

    new_odds_a = format!("{:.3}", new_odds_a).parse::<f64>()?;
    new_odds_b = format!("{:.3}", new_odds_b).parse::<f64>()?;

    let odds = odds_repository
        .create(&OddsCreate {
            game_match_id: match_uuid,
            odds_a: new_odds_a,
            odds_b: new_odds_b,
        })
        .await?;
    Ok(odds)
}

pub async fn get_active_bets_by_user_id(
    mut bet_repository: BetRepository,
    mut match_repository: GameMatchRepository,
    mut game_repository: GameRepository,
    user_id: Uuid,
) -> AppResult<Vec<Bet>> {
    let active_user_bets = bet_repository
        .read_many(&BetGetByUserId { user_id })
        .await?;

    let mut active_bets = Vec::new();

    for bet in active_user_bets {
        let Ok(game_match) = match_repository
            .read_one(&GameMatchGetById {
                id: bet.game_match_id,
            })
            .await
        else {
            continue;
        };

        if game_match.status != GameMatchStatus::Live {
            continue;
        }

        let predicted_team = match bet.expected_outcome {
            GameMatchOutcome::WinA => game_match.name_a.clone(),
            _ => game_match.name_b.clone(),
        };

        let bet_date = format_date_time_string_with_seconds(&bet.created_at);

        let Ok(game) = game_repository
            .read_one(&GameGetById {
                id: game_match.game_id,
            })
            .await
        else {
            continue;
        };

        active_bets.push(Bet {
            game_name: game.name,
            match_id: bet.game_match_id,
            team_a: game_match.name_a,
            team_b: game_match.name_b,
            predicted_team: predicted_team.to_string(),
            bet_amount: bet.amount,
            date: bet_date,
        });
    }

    Ok(active_bets)
}

pub async fn get_active_bets_handler(
    auth_session: AuthSession,
    Extension(error_web_socket): Extension<ExtensionWebSocketError>,
    Extension(match_repo): Extension<GameMatchRepository>,
    Extension(bet_repo): Extension<BetRepository>,
    Extension(game_repo): Extension<GameRepository>,
) -> AppResult<Html<String>> {
    let user = auth::is_logged_in(auth_session)?;

    let bets = get_active_bets_by_user_id(
        bet_repo.clone(),
        match_repo.clone(),
        game_repo.clone(),
        user.id,
    )
    .await
    .or_else(|e| {
        show_popup_error(
            "Failed to get active bets",
            e,
            user.id,
            error_web_socket.clone(),
        )
    })?;

    let active_bets_template = ActiveBets { bets }.render()?;

    Ok(Html(active_bets_template))
}
