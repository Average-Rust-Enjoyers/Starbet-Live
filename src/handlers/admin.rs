use crate::{
    common::{DbReadAll, DbUpdateOne},
    models::{
        game::GameFilter,
        game_match::{self, GameMatchCreate, GameMatchGetById, GameMatchStatus},
        game_match_outcome::GameMatchOutcome,
    },
    repositories::game_match::GameMatchRepository,
    templates::{AdminPanel, AdminPanelMatch},
    DbCreate, DbReadMany, DbReadOne, GameRepository,
};
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension, Form,
};
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
    Form(payload): Form<GameMatchCreate>,
) -> impl IntoResponse {
    if payload.ends_at < payload.starts_at {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "match end cannot be before start",
        )
            .into_response();
    }

    if game_match_repo.create(&payload).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    (
        StatusCode::CREATED,
        Html("<script>window.location = '/admin'</script>"),
    )
        .into_response()
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

pub async fn gamematch_update_handler(
    Path(match_id): Path<Uuid>,
    Extension(mut game_match_repo): Extension<GameMatchRepository>,
    Form(GameMatchUpdateData { action }): Form<GameMatchUpdateData>,
) -> impl IntoResponse {
    let game_match = game_match_repo
        .read_one(&GameMatchGetById { id: match_id })
        .await;

    if game_match.is_err() {
        return StatusCode::NOT_FOUND.into_response();
    }

    let game_match = game_match.unwrap();

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

    let game_match = game_match_repo
        .update(&game_match::GameMatchUpdate {
            id: game_match.id,
            name_a: None,
            name_b: None,
            starts_at: None,
            ends_at: None,
            outcome: action.into(),
            status: action.into(),
        })
        .await;

    if game_match.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let template = AdminPanelMatch {
        game_match: game_match.unwrap(),
    };

    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html)).into_response()
}
