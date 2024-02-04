use askama::Template;
use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    Extension, Form,
};
use serde::Deserialize;

use crate::{
    error::AppResult,
    repositories::user::UserRepository,
    templates::TextField,
    validators::{
        validate_confirm_password, validate_email, validate_name, validate_password,
        validate_username,
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
    Extension(mut user_repository): Extension<UserRepository>,
    Path(Params { field }): Path<Params>,
    Form(payload): Form<RegisterFormData>,
) -> AppResult<impl IntoResponse> {
    let (_, textfield) = validate_and_build(&field, &payload, &mut user_repository).await;
    Ok(Html(textfield.render()?))
}

pub async fn validate_and_build<'a>(
    field: &'a str,
    payload: &RegisterFormData,
    user_repository: &mut UserRepository,
) -> (bool, TextField<'a>) {
    let (value, error_message) = match field {
        "username" => validate_username(payload.username.clone(), user_repository).await,
        "first-name" => validate_name(payload.first_name.clone()),
        "last-name" => validate_name(payload.last_name.clone()),
        "email" => validate_email(payload.email.clone(), user_repository).await,
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
