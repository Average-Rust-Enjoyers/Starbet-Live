use crate::{
    common::{DbGetLatest, DbReadAll, DbUpdateOne},
    config::DEFAULT_ODDS_VALUE,
    models::{
        game::GameFilter,
        game_match::{self, GameMatchCreate, GameMatchGetById, GameMatchStatus},
        game_match_outcome::GameMatchOutcome,
        odds::{Odds, OddsCreate, OddsGetByGameMatchId},
    },
    repositories::{game::GameRepository, game_match::GameMatchRepository, odds::OddsRepository},
    routers::HxRedirect,
    templates::{AdminPanel, AdminPanelMatch},
    DbCreate, DbReadMany, DbReadOne,
};
use askama::Template;
use axum::{
    extract::Path,
    http::{StatusCode, Uri},
    response::{Html, IntoResponse},
    Extension, Form,
};
use rand::Rng;
use serde::Deserialize;
use uuid::Uuid;

pub async fn admin_handler(
    Extension(mut game_repo): Extension<GameRepository>,
    Extension(mut game_match_repo): Extension<GameMatchRepository>,
) -> impl IntoResponse {
    let games = game_repo
        .read_many(&GameFilter {
            name: None,
            genre: None,
        })
        .await
        .unwrap();
    let matches = game_match_repo.read_all().await.unwrap();

    let template = AdminPanel { games, matches };

    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn new_gamematch_handler(
    Extension(mut game_match_repo): Extension<GameMatchRepository>,
    Form(mut payload): Form<GameMatchCreate>,
) -> impl IntoResponse {
    payload.cloudbet_id = None;

    if payload.ends_at < payload.starts_at {
        return (StatusCode::BAD_REQUEST, "match end cannot be before start").into_response();
    }

    if game_match_repo.create(&payload).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    HxRedirect(Uri::from_static("/admin")).into_response()
}

#[derive(Deserialize, Copy, Clone)]
#[serde(rename_all = "snake_case")]
enum GameMatchUpdateAction {
    Start,
    WinA,
    WinB,
    Cancel,
}

impl From<GameMatchUpdateAction> for Option<game_match::GameMatchStatus> {
    fn from(value: GameMatchUpdateAction) -> Self {
        match value {
            GameMatchUpdateAction::Cancel => Some(GameMatchStatus::Canceled),
            GameMatchUpdateAction::WinA | GameMatchUpdateAction::WinB => {
                Some(GameMatchStatus::Finished)
            }
            GameMatchUpdateAction::Start => Some(GameMatchStatus::Live),
        }
    }
}

impl From<GameMatchUpdateAction> for Option<GameMatchOutcome> {
    fn from(value: GameMatchUpdateAction) -> Self {
        match value {
            GameMatchUpdateAction::WinA => Some(GameMatchOutcome::WinA),
            GameMatchUpdateAction::WinB => Some(GameMatchOutcome::WinB),
            _ => None,
        }
    }
}

#[derive(Deserialize)]
pub struct GameMatchUpdateData {
    action: GameMatchUpdateAction,
}

pub async fn gamematch_random_odds_handler(
    Path(match_id): Path<Uuid>,
    Extension(mut game_match_repo): Extension<GameMatchRepository>,
    Extension(mut odds_repo): Extension<OddsRepository>,
) -> impl IntoResponse {
    let game_match = game_match_repo
        .read_one(&GameMatchGetById { id: match_id })
        .await;

    let Ok(game_match) = game_match else {
        return StatusCode::NOT_FOUND.into_response();
    };

    if game_match.status != GameMatchStatus::Live {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let odds = odds_repo
        .get_latest(&OddsGetByGameMatchId {
            game_match_id: game_match.id,
        })
        .await
        .unwrap_or(Odds {
            id: game_match.id,
            game_match_id: game_match.id,
            odds_a: DEFAULT_ODDS_VALUE,
            odds_b: DEFAULT_ODDS_VALUE,
            created_at: game_match.created_at,
            deleted_at: None,
        });

    let mut odds_a = odds.odds_a;
    let mut odds_b = odds.odds_b;

    let odds_max = f64::max(1.1f64, f64::max(odds_a, odds_b));
    let rng = rand::thread_rng().gen_range(0.1..=odds_max - 1f64);

    if odds_b - rng > 1f64 {
        odds_a += rng;
        odds_b -= rng;
    } else if odds_a - rng > 1f64 {
        odds_a -= rng;
        odds_b += rng;
    } else {
        odds_a += rng;
        odds_b += rng;
    }

    if odds_repo
        .create(&OddsCreate {
            game_match_id: game_match.id,
            odds_a,
            odds_b,
        })
        .await
        .is_err()
    {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let template = AdminPanelMatch { game_match };

    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html)).into_response()
}

pub async fn gamematch_update_handler(
    Path(match_id): Path<Uuid>,
    Extension(mut game_match_repo): Extension<GameMatchRepository>,
    Form(GameMatchUpdateData { action }): Form<GameMatchUpdateData>,
) -> impl IntoResponse {
    let game_match = game_match_repo
        .read_one(&GameMatchGetById { id: match_id })
        .await;

    let Ok(game_match) = game_match else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let valid_action = matches!(
        (game_match.status, action),
        (GameMatchStatus::Pending, GameMatchUpdateAction::Start)
            | (GameMatchStatus::Live, GameMatchUpdateAction::WinA)
            | (GameMatchStatus::Live, GameMatchUpdateAction::WinB)
            | (GameMatchStatus::Pending, GameMatchUpdateAction::Cancel)
            | (GameMatchStatus::Live, GameMatchUpdateAction::Cancel)
    );

    if !valid_action {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let Ok(game_match) = game_match_repo
        .update(&game_match::GameMatchUpdate {
            id: game_match.id,
            name_a: None,
            name_b: None,
            starts_at: None,
            ends_at: None,
            outcome: action.into(),
            status: action.into(),
        })
        .await
    else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let template = AdminPanelMatch { game_match };

    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html)).into_response()
}
