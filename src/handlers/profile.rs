use crate::templates::ProfilePage;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn profile_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Html(ProfilePage {}.render().unwrap()).into_response(),
    )
}
