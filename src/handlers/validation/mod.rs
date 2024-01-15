use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Form,
};

use crate::{
    templates::TextField,
    validators::register::{
        validate_email, validate_password, validate_username, vlaidate_name, RegisterFormData,
    },
};

fn create_reply_html(name: &str, value: &str, error_message: &str) -> String {
    (TextField {
        name,
        value,
        error_message,
    })
    .render()
    .unwrap()
}

pub async fn username_handler(Form(payload): Form<RegisterFormData>) -> impl IntoResponse {
    let (value, error_message) = validate_username(payload.username.clone());
    let reply_html = create_reply_html("username", &value, &error_message);
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn first_name_handler(Form(payload): Form<RegisterFormData>) -> impl IntoResponse {
    let (value, error_message) = vlaidate_name(payload.first_name.clone());
    let reply_html = create_reply_html("first-name", &value, &error_message);
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn last_name_handler(Form(payload): Form<RegisterFormData>) -> impl IntoResponse {
    let (value, error_message) = vlaidate_name(payload.last_name.clone());
    let reply_html = create_reply_html("last-name", &value, &error_message);
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn email_handler(Form(payload): Form<RegisterFormData>) -> impl IntoResponse {
    let (value, error_message) = validate_email(payload.email.clone());
    let reply_html = create_reply_html("email", &value, &error_message);
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn password_handler(Form(payload): Form<RegisterFormData>) -> impl IntoResponse {
    let (value, error_message) = validate_password(payload.password.clone());
    let reply_html = create_reply_html("password", &value, &error_message);
    (StatusCode::OK, Html(reply_html).into_response())
}
