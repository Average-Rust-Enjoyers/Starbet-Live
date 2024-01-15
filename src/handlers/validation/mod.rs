use askama::Template;
use axum::{
    extract::Path,
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

#[derive(serde::Deserialize)]
pub struct Params {
    pub field_name: String,
}

pub async fn validation_handler(
    Path(Params { field_name }): Path<Params>,
    Form(payload): Form<RegisterFormData>,
) -> impl IntoResponse {
    let (value, error_message) = match field_name.as_str() {
        "username" => validate_username(payload.username),
        "first-name" => vlaidate_name(payload.first_name),
        "last-name" => vlaidate_name(payload.last_name),
        "email" => validate_email(payload.email),
        "password" => validate_password(payload.password),
        _ => (String::new(), String::new()),
    };
    let reply_html = create_reply_html(&field_name, &value, &error_message);
    (StatusCode::OK, Html(reply_html).into_response())
}
