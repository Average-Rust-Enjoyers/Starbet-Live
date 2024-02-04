use crate::{
    auth::{self, AuthSession},
    common::{helpers::generate_error_message_template, DbReadAll},
    error::AppResult,
    models::extension_web_socket::ExtensionWebSocketError,
    repositories::{bet::BetRepository, game::GameRepository, game_match::GameMatchRepository},
    templates::{ActiveBets, Dashboard, Menu, MenuItem, UserBalance, UserNav, UserSend},
};
use askama::Template;
use axum::{response::Html, Extension};

use super::bet::get_active_bets_by_user_id;

pub async fn dashboard_handler(
    auth_session: AuthSession,
    Extension(error_web_socket): Extension<ExtensionWebSocketError>,
    Extension(mut game_repository): Extension<GameRepository>,
    Extension(match_repository): Extension<GameMatchRepository>,
    Extension(bet_repository): Extension<BetRepository>,
) -> AppResult<Html<String>> {
    let user = auth::is_logged_in(auth_session)?;

    let user_id = user.id;
    let user_send = UserSend::from(&user);

    let games = game_repository.read_all().await?;
    // =======
    //         let _ = error_web_socket
    //             .tx
    //             .send_async(generate_error_message_template(
    //                 "Failed to get games",
    //                 user_id,
    //             ))
    //             .await;
    // >>>>>>> main

    let active_user_bets = get_active_bets_by_user_id(
        bet_repository.clone(),
        match_repository.clone(),
        game_repository.clone(),
        user_id,
    )
    .await?;
    // else {
    //     let _ = error_web_socket
    //         .tx
    //         .send_async(generate_error_message_template(
    //             "Failed to get active bets",
    //             user_id,
    //         ))
    //         .await;

    //     return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    // };

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
        profile_picture: user.profile_picture,
        user_balance,
    };

    let template = Dashboard {
        user: user_send,
        menu,
        active_bets,
        user_nav,
    };

    let reply_html = template.render()?;
    Ok(Html(reply_html))
}
