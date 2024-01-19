use askama::Template;
use axum::{
    extract::Request,
    http::StatusCode,
    response::{Html, IntoResponse},
    Extension, Form,
};

use super::validation::{validate_and_build, RegisterFormData};
use crate::{
    common::repository::DbCreate,
    models::user::UserCreate,
    repositories::user::UserRepository,
    templates::{RegisterForm, RegisterPage, TextField},
};

const FIELDS: [&str; 6] = [
    "username",
    "first-name",
    "last-name",
    "email",
    "password",
    "confirm-password",
];

pub async fn register_page_handler(req: Request) -> impl IntoResponse {
    let form = RegisterForm {
        username: TextField::new(FIELDS[0]),
        first_name: TextField::new(FIELDS[1]),
        last_name: TextField::new(FIELDS[2]),
        email: TextField::new(FIELDS[3]),
        password: TextField::new(FIELDS[4]),
        confirm_password: TextField::new(FIELDS[5]),
    };

    // If the reqest came from HTMX, render only the form
    // and don't do a full page refresh
    let reply_html = if req.headers().contains_key("referer") {
        form.render().unwrap()
    } else {
        RegisterPage { form }.render().unwrap()
    };
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn register_submission_handler(
    Extension(mut user_repository): Extension<UserRepository>,
    Form(payload): Form<RegisterFormData>,
) -> impl IntoResponse {
    let (all_valid, form_fields): (bool, Vec<TextField>) = FIELDS
        .iter()
        .map(|field| validate_and_build(field, &payload))
        .fold(
            (true, Vec::new()),
            |(valid_acc, mut fields_acc), (valid, field)| {
                // Perform logical AND on all bools and collect all TextFields
                fields_acc.push(field);
                (valid_acc && valid, fields_acc)
            },
        );

    if !all_valid {
        let form = RegisterForm {
            username: form_fields[0].clone(),
            first_name: form_fields[1].clone(),
            last_name: form_fields[2].clone(),
            email: form_fields[3].clone(),
            password: form_fields[4].clone(),
            confirm_password: form_fields[5].clone(),
        }
        .render()
        .unwrap();
        return (StatusCode::OK, Html(form).into_response());
    }

    let _user = user_repository
        .create(&UserCreate::from(&payload))
        .await
        .expect("Failed to create user");

    (StatusCode::OK, Html("Hi").into_response())
}
