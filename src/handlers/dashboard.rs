use crate::{
    auth::AuthSession,
    templates::{Dashboard, Menu, MenuItem, UserSend},
};
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};

/// # Panics
pub async fn dashboard_handler(auth_session: AuthSession) -> impl IntoResponse {
    let user = match auth_session.user {
        Some(user) => UserSend::from(&user),
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let games = game_repository.read_all().await.unwrap();

    let menu_items: Vec<MenuItem> = games
        .iter()
        .map(|game| MenuItem {
            name: game.name.clone(),
            game_id: game.id,
            active: false,
        })
        .collect();

    let menu = Menu { games: menu_items };

    let template = Dashboard { user, menu };

    let reply_html = template.render().unwrap();
    // TODO: status code?
    Html(reply_html).into_response()
}
