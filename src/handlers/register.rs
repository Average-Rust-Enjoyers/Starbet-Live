use askama::Template;
use axum::{
    http::{StatusCode, Uri},
    response::{Html, IntoResponse},
    Extension, Form,
};

use super::validation::{validate_and_build, RegisterFormData};
use crate::{
    auth::AuthSession,
    common::repository::DbCreate,
    models::user::Credentials,
    repositories::user::UserRepository,
    routers::HxRedirect,
    templates::{RegisterPage, ServerErrorPage, TextField},
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
    mut auth_session: AuthSession,
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
        return (StatusCode::OK, Html(form).into_response()).into_response();
    }

    let credentials = Credentials {
        email: payload.email.clone(),
        password: payload.password.clone(),
        next: None,
    };

    match user_repository.create(&payload.into()).await {
        Ok(_) => {
            match auth_session.authenticate(credentials).await {
                Ok(Some(user)) => {
                    if auth_session.login(&user).await.is_err() {
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                }
                _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            };

            HxRedirect(Uri::from_static("/dashboard")).into_response()
        }
        Err(_) => (
            StatusCode::OK,
            Html(ServerErrorPage {}.render().unwrap()).into_response(),
        )
            .into_response(),
    }
}
