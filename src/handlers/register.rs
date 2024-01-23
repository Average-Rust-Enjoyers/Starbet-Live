use askama::Template;
use axum::{
    http::{HeaderValue, StatusCode},
    response::{Html, IntoResponse},
    Extension, Form,
};

use super::validation::{validate_and_build, RegisterFormData};
use crate::{
    common::repository::DbCreate,
    repositories::user::UserRepository,
    templates::{LoginPage, RegisterPage, ServerErrorPage, TextField},
};

const FIELDS: [&str; 6] = [
    "username",
    "first-name",
    "last-name",
    "email",
    "password",
    "confirm-password",
];

pub async fn register_page_handler() -> impl IntoResponse {
    let form = RegisterPage {
        username: TextField::new(FIELDS[0]),
        first_name: TextField::new(FIELDS[1]),
        last_name: TextField::new(FIELDS[2]),
        email: TextField::new(FIELDS[3]),
        password: TextField::new(FIELDS[4]),
        confirm_password: TextField::new(FIELDS[5]),
    };

    (StatusCode::OK, Html(form.render().unwrap()).into_response())
}

pub async fn register_submission_handler(
    Extension(mut user_repository): Extension<UserRepository>,
    Form(payload): Form<RegisterFormData>,
) -> impl IntoResponse {
    let (mut all_valid, mut form_fields) = (true, Vec::new());

    for field in &FIELDS {
        let (valid, field) = validate_and_build(field, &payload, &mut user_repository).await;
        all_valid = all_valid && valid;
        form_fields.push(field);
    }

    if !all_valid {
        let form = RegisterPage {
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

    if (user_repository.create(&payload.into()).await).is_ok() {
        let mut response = Html(LoginPage {}.render().unwrap()).into_response();
        response
            .headers_mut()
            .insert("HX-Redirect", HeaderValue::from_static("/login"));
        (StatusCode::CREATED, response)
    } else {
        (
            StatusCode::OK,
            Html(ServerErrorPage {}.render().unwrap()).into_response(),
        )
    }
}
