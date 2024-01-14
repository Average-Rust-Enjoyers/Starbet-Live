use askama::Template;
use axum::{
    extract::Json,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::Deserialize;

use crate::templates::TextField;

#[derive(Deserialize)]
pub struct Username {
    username: String,
}

pub fn is_valid_username(username: &str) -> (bool, &str) {
    if username.len() < 3 {
        return (false, "Username must be at least 3 characters long");
    }
    if username.len() > 20 {
        return (false, "Username must be at most 20 characters long");
    }
    if !username
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.')
    {
        return (
            false,
            "Username can only contain numbers, characters, underscores and dots",
        );
    }
    if username.starts_with('.') || username.ends_with('.') {
        return (false, "Username cannot start or end with a dot");
    }

    //TODO: check if username is already taken

    (true, "")
}

pub async fn username_handler(Json(payload): Json<Username>) -> impl IntoResponse {
    let (is_valid, error_message) = is_valid_username(&payload.username);

    println!("is_valid: {is_valid}, error_message: {error_message}");

    let reply_html = (TextField {
        name: "username",
        value: &payload.username,
        is_valid,
        error_message,
    })
    .render()
    .unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
