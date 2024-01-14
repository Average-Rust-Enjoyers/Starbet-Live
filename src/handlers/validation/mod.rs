use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Form,
};

use crate::{
    templates::TextField,
    validators::register::{validate_username, RegisterFormData},
};

pub async fn username_handler(Form(payload): Form<RegisterFormData>) -> impl IntoResponse {
    let (value, error_message) = validate_username(payload.username.clone());
    let reply_html = (TextField {
        name: "username",
        value: &value,
        error_message: &error_message,
    })
    .render()
    .unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
