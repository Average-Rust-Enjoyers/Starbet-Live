use crate::{
    auth,
    common::helpers::format_date_time_string_with_seconds,
    models::{
        bet::{BetGetByUserId, BetStatus},
        game::GameGetById,
        game_match,
        game_match_outcome::GameMatchOutcome,
        odds,
    },
    repositories::{bet::BetRepository, game_match::GameMatchRepository, odds::OddsRepository},
    templates::{BetHistory, BetHistoryBet, EditProfilePage, ProfilePage, TextField},
    DbReadMany, DbReadOne, GameRepository,
};
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};

pub async fn profile_handler(auth_session: auth::AuthSession) -> impl IntoResponse {
    let Some(user) = auth_session.user else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    (
        StatusCode::OK,
        Html(ProfilePage::from(user).render().unwrap()),
    )
        .into_response()
}

pub async fn bet_history_handler(
    auth_session: auth::AuthSession,
    Extension(mut bet_repo): Extension<BetRepository>,
    Extension(mut match_repo): Extension<GameMatchRepository>,
    Extension(mut game_repo): Extension<GameRepository>,
    Extension(mut odds_repo): Extension<OddsRepository>,
) -> impl IntoResponse {
    let Some(user) = auth_session.user else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let user_bets = bet_repo
        .read_many(&BetGetByUserId { user_id: user.id })
        .await
        .unwrap();

    let mut bet_history = Vec::new();

    for bet in &user_bets {
        if bet.status != BetStatus::Won && bet.status != BetStatus::Lost {
            continue;
        }

        let won = bet.status == BetStatus::Won;

        let game_match = match_repo
            .read_one(&game_match::GameMatchGetById {
                id: bet.game_match_id,
            })
            .await
            .unwrap();

        let date = format_date_time_string_with_seconds(&bet.created_at);

        let game_name = game_repo
            .read_one(&GameGetById {
                id: game_match.game_id,
            })
            .await
            .unwrap()
            .name;

        let odds = odds_repo
            .read_one(&odds::OddsGetById { id: bet.odds_id })
            .await
            .unwrap();

        let (predicted_team, multiplier, won_amount) = match bet.expected_outcome {
            GameMatchOutcome::WinA => (
                game_match.name_a.clone(),
                odds.odds_a,
                odds.odds_a * f64::from(bet.amount),
            ),
            _ => (
                game_match.name_b.clone(),
                odds.odds_b,
                odds.odds_b * f64::from(bet.amount),
            ),
        };

        #[allow(clippy::cast_possible_truncation)]
        let won_amount = if won { won_amount as i32 } else { 0 };

        let bet_history_bet = BetHistoryBet {
            game_name,
            team_a: game_match.name_a,
            team_b: game_match.name_b,
            predicted_team,
            bet_amount: bet.amount,
            multiplier,
            won_amount,
            won,
            date,
        };

        bet_history.push(bet_history_bet);
    }

    (
        StatusCode::OK,
        Html(BetHistory { bets: bet_history }.render().unwrap()),
    )
        .into_response()
}

const FIELDS: [&str; 4] = ["username", "first-name", "last-name", "email"];

pub async fn get_edit_profile_handler(auth_session: auth::AuthSession) -> impl IntoResponse {
    let Some(user) = auth_session.user else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    (
        StatusCode::OK,
        Html(
            EditProfilePage {
                username: TextField::new(FIELDS[0]),
                first_name: TextField::new(FIELDS[1]),
                last_name: TextField::new(FIELDS[2]),
                email: TextField::new(FIELDS[3]),
            }
            .render()
            .unwrap(),
        ),
    )
        .into_response()
}
