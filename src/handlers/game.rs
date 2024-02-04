use crate::{
    auth::{self, AuthSession},
    common::{helpers::format_date_time_string_without_seconds, DbGetLatest, DbReadByForeignKey},
    error::AppResult,
    models::{game::GameGetById, game_match::GameMatchStatus, odds::OddsGetByGameMatchId},
    repositories::{game::GameRepository, game_match::GameMatchRepository, odds::OddsRepository},
    templates::{Game, Match, Menu, MenuItem, UpcomingMatch},
};
use askama::Template;
use axum::{extract::Path, response::Html, Extension};

use serde::Deserialize;

use uuid::Uuid;

use crate::common::repository::{DbReadAll, DbReadOne};

#[derive(Deserialize)]
pub struct GameId {
    pub game_id: String,
}

pub async fn game_handler(
    auth_session: AuthSession,
    Extension(mut game_repository): Extension<GameRepository>,
    Extension(mut game_match_repo): Extension<GameMatchRepository>,
    Extension(mut odds_repo): Extension<OddsRepository>,
    Path(GameId { game_id }): Path<GameId>,
) -> AppResult<Html<String>> {
    auth::is_logged_in(auth_session)?;

    let game = game_repository
        .read_one(&GameGetById {
            id: Uuid::parse_str(&game_id.clone())?,
        })
        .await?;

    let matches = game_match_repo.get_by_foreign_key(&game.id).await?;

    let mut matches_to_render = Vec::new();
    let mut upcoming_matches_to_render = Vec::new();

    for game_match in matches {
        match game_match.status {
            GameMatchStatus::Live => {
                let latest_odds = odds_repo
                    .get_latest(&OddsGetByGameMatchId {
                        game_match_id: game_match.id,
                    })
                    .await?;

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

    let menu_items = game_repository.read_all().await?;

    let menu_items = menu_items
        .iter()
        .map(|game| MenuItem {
            name: game.name.clone(),
            game_id: game.id,
            active: game.id.clone().to_string() == game_id,
        })
        .collect();

    let menu = Menu { games: menu_items }.render()?;
    let game = template.render()?;

    let response = format!("{menu}{game}");

    Ok(Html(response))
}
