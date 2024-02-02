use crate::{
    auth::AuthSession,
    common::DbReadAll,
    repositories::{bet::BetRepository, game::GameRepository, game_match::GameMatchRepository},
    templates::{ActiveBets, Dashboard, Menu, MenuItem, UserBalance, UserNav, UserSend},
};
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};

use super::bet::get_active_bets_by_user_id;

/// # Panics
pub async fn dashboard_handler(
    auth_session: AuthSession,
    Extension(mut game_repository): Extension<GameRepository>,
    Extension(match_repository): Extension<GameMatchRepository>,
    Extension(bet_repository): Extension<BetRepository>,
) -> impl IntoResponse {
    let Some(user) = auth_session.user else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let user_id = user.id;
    let user_send = UserSend::from(&user);

    let games = game_repository.read_all().await.unwrap();

    let active_user_bets = get_active_bets_by_user_id(
        bet_repository.clone(),
        match_repository.clone(),
        game_repository.clone(),
        user_id,
    )
    .await;

    let menu_items: Vec<MenuItem> = games
        .iter()
        .map(|game| MenuItem {
            name: game.name.clone(),
            game_id: game.id,
            active: false,
        })
        .collect();

    let menu = Menu { games: menu_items };
    let active_bets = ActiveBets {
        bets: active_user_bets,
    };

    let user_balance = UserBalance {
        balance: user.balance,
    };

    let user_nav = UserNav {
        username: user.username,
        user_balance,
    };

    let template = Dashboard {
        user: user_send,
        menu,
        active_bets,
        user_nav,
    };

    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html)).into_response()
}
