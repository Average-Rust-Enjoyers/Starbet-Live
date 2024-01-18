use crate::templates::{Dashboard, Menu, MenuItem, UserSend};
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn dashboard_handler() -> impl IntoResponse {
    let user = UserSend {
        username: "Eric Cartman".to_string(),
        email: "eric.cartman@southpark.com".to_string(),
        name: "Eric".to_string(),
        surname: "Cartman".to_string(),
        profile_picture: "this_is_my_picture.jpg".to_string(),
        balance: 69420,
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
    (StatusCode::OK, Html(reply_html).into_response())
}
