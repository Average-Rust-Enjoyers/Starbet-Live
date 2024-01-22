use crate::templates::LoginPage;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn login_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Html(LoginPage {}.render().unwrap()).into_response(),
    )
}
