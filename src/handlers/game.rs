use crate::{
    auth::AuthSession,
    common::{
        helpers::{format_date_time_string_without_seconds, generate_error_message_template},
        DbGetLatest, DbReadByForeignKey,
    },
    models::{
        extension_web_socket::ExtensionWebSocketError, game::GameGetById,
        game_match::GameMatchStatus, odds::OddsGetByGameMatchId,
    },
    repositories::{game::GameRepository, game_match::GameMatchRepository, odds::OddsRepository},
    templates::{Game, Match, Menu, MenuItem, UpcomingMatch},
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

use crate::common::repository::{DbReadAll, DbReadOne};

#[derive(Deserialize)]
pub struct GameId {
    pub game_id: String,
}

pub async fn game_handler(
    auth_session: AuthSession,
    Extension(error_web_socket): Extension<ExtensionWebSocketError>,
    Extension(mut game_repository): Extension<GameRepository>,
    Extension(mut game_match_repo): Extension<GameMatchRepository>,
    Extension(mut odds_repo): Extension<OddsRepository>,
    Path(GameId { game_id }): Path<GameId>,
) -> impl IntoResponse {
    let Some(user) = auth_session.user else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let Ok(game) = game_repository
        .read_one(&GameGetById {
            id: Uuid::parse_str(&game_id.clone()).unwrap(),
        })
        .await
    else {
        let _ = error_web_socket
            .tx
            .send_async(generate_error_message_template(
                "Failed to get game",
                user.id,
            ))
            .await;

        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let Ok(matches) = game_match_repo.get_by_foreign_key(&game.id).await else {
        let _ = error_web_socket
            .tx
            .send_async(generate_error_message_template(
                "Failed to get matches",
                user.id,
            ))
            .await;

        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let mut matches_to_render = Vec::new();
    let mut upcoming_matches_to_render = Vec::new();

    for game_match in matches {
        match game_match.status {
            GameMatchStatus::Live => {
                let Ok(latest_odds) = odds_repo
                    .get_latest(&OddsGetByGameMatchId {
                        game_match_id: game_match.id,
                    })
                    .await
                else {
                    let _ = error_web_socket
                        .tx
                        .send_async(generate_error_message_template(
                            "Failed to get odds",
                            user.id,
                        ))
                        .await;

                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                };

                matches_to_render.push(Match {
                    match_id: game_match.id,
                    team_a: game_match.name_a,
                    team_b: game_match.name_b,
                    current_odds: latest_odds,
                });
            }
            _ => upcoming_matches_to_render.push(UpcomingMatch {
                match_id: game_match.id,
                team_a: game_match.name_a,
                team_b: game_match.name_b,
                date: format_date_time_string_without_seconds(&game_match.starts_at),
            }),
        }
    }

    let template = Game {
        game_name: game.name.clone(),
        matches: matches_to_render,
        upcoming_matches: upcoming_matches_to_render,
        game_id: game_id.clone(),
    };

    let Ok(menu_items) = game_repository.read_all().await else {
        let _ = error_web_socket
            .tx
            .send_async(generate_error_message_template(
                "Failed to get games",
                user.id,
            ))
            .await;

        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let menu_items = menu_items
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
    (StatusCode::OK, Html(response)).into_response()
}
