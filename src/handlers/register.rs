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
    error::AppResult,
    models::user::Credentials,
    repositories::user::UserRepository,
    routers::HxRedirect,
    templates::{RegisterPage, TextField},
};

const FIELDS: [&str; 6] = [
    "username",
    "first-name",
    "last-name",
    "email",
    "password",
    "confirm-password",
];

pub async fn register_page_handler() -> AppResult<Html<String>> {
    let form = RegisterPage {
        username: TextField::new(FIELDS[0]),
        first_name: TextField::new(FIELDS[1]),
        last_name: TextField::new(FIELDS[2]),
        email: TextField::new(FIELDS[3]),
        password: TextField::new(FIELDS[4]),
        confirm_password: TextField::new(FIELDS[5]),
    };

    Ok(Html(form.render()?))
}

pub async fn register_submission_handler(
    mut auth_session: AuthSession,
    Extension(mut user_repository): Extension<UserRepository>,
    Form(payload): Form<RegisterFormData>,
) -> AppResult<impl IntoResponse> {
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
        .render()?;
        return Ok((StatusCode::OK, Html(form).into_response()).into_response());
    }

    let credentials = Credentials {
        email: payload.email.clone(),
        password: payload.password.clone(),
        next: None,
    };

    user_repository.create(&payload.into()).await?;

    let user = auth_session.authenticate(credentials).await?;
    if let Some(user) = user {
        auth_session.login(&user).await?;
    }

    Ok(HxRedirect(Uri::from_static("/dashboard")).into_response())
}
