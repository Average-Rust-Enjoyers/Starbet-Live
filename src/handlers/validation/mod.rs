use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    Form,
};
use serde::Deserialize;

use crate::{
    templates::TextField,
    validators::register::{
        validate_confirm_password, validate_email, validate_password, validate_username,
        vlaidate_name,
    },
};

#[derive(serde::Deserialize)]
pub struct Params {
    pub field: String,
}

#[derive(Deserialize)]
pub struct RegisterFormData {
    pub username: String,
    #[serde(rename = "first-name")]
    pub first_name: String,
    #[serde(rename = "last-name")]
    pub last_name: String,
    pub email: String,
    pub password: String,
    #[serde(rename = "confirm-password")]
    pub confirm_password: String,
}

pub async fn validation_handler(
    Path(Params { field }): Path<Params>,
    Form(payload): Form<RegisterFormData>,
) -> impl IntoResponse {
    let (_, textfield) = validate_and_build(&field, &payload);
    (
        StatusCode::OK,
        Html(textfield.render().unwrap()).into_response(),
    )
}

pub fn validate_and_build<'a>(field: &'a str, payload: &RegisterFormData) -> (bool, TextField<'a>) {
    let (value, error_message) = match field {
        "username" => validate_username(payload.username.clone()),
        "first-name" => vlaidate_name(payload.first_name.clone()),
        "last-name" => vlaidate_name(payload.last_name.clone()),
        "email" => validate_email(payload.email.clone()),
        "password" => validate_password(payload.password.clone()),
        "confirm-password" => {
            validate_confirm_password(&payload.password, payload.confirm_password.clone())
        }
        _ => (String::new(), "Invalid field".to_string()),
    };

    (
        error_message.is_empty(),
        TextField {
            name: field,
            value,
            error_message,
        },
    )
}
