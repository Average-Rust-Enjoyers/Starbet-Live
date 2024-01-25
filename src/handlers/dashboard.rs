use crate::{
    auth::AuthSession,
    templates::{Dashboard, Menu, MenuItem, UserSend},
};
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

/// # Panics
pub async fn dashboard_handler(auth_session: AuthSession) -> impl IntoResponse {
    let user = match auth_session.user {
        Some(user) => UserSend::from(&user),
        None => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let menu_items = vec![
        MenuItem {
            name: "CS:GO".to_string(),
            active: false,
        },
        MenuItem {
            name: "Dota 2".to_string(),
            active: false,
        },
        MenuItem {
            name: "LoL".to_string(),
            active: false,
        },
        MenuItem {
            name: "Valorant".to_string(),
            active: false,
        },
    ];

    let menu = Menu { games: menu_items };

    let template = Dashboard { user, menu };

    let reply_html = template.render().unwrap();
    // TODO: status code?
    Html(reply_html).into_response()
}
