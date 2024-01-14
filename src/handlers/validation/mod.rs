use axum::{
    extract::Json,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Username {
    username: String,
}

pub async fn username_handler(Json(payload): Json<Username>) -> impl IntoResponse {
    let value = payload.username;
    format!("Hello, {value}!");
    let reply_html = "validation handler";
    (StatusCode::OK, Html(reply_html).into_response())
}
