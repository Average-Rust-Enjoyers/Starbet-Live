use crate::templates::{Dashboard, UserSend};
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn dashboard_handler() -> impl IntoResponse {
    // TODO: Get the menu items from the database
    let menu_items = vec![
        "CS:GO".to_string(),
        "Dota 2".to_string(),
        "LoL".to_string(),
        "Valorant".to_string(),
    ];

    let user = UserSend {
        username: "Eric Cartman".to_string(),
        email: "eric.cartman@southpark.com".to_string(),
        name: "Eric".to_string(),
        surname: "Cartman".to_string(),
        profile_picture: "this_is_my_picture.jpg".to_string(),
        balance: 69420,
    };

    let template = Dashboard {
        items: menu_items,
        user,
    };

    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
