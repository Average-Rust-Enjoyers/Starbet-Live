use crate::{
    repositories::game::GameRepository,
    templates::{Dashboard, Menu, MenuItem, UserSend},
};
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension,
};

use crate::common::repository::DbReadAll;

pub async fn dashboard_handler(
    Extension(mut game_repository): Extension<GameRepository>,
) -> impl IntoResponse {
    let user = UserSend {
        username: "Eric Cartman".to_string(),
        email: "eric.cartman@southpark.com".to_string(),
        name: "Eric".to_string(),
        surname: "Cartman".to_string(),
        profile_picture: "this_is_my_picture.jpg".to_string(),
        balance: 69420,
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
    (StatusCode::OK, Html(reply_html).into_response())
}
