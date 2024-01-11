use crate::templates::LandingPageTemplate;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn index_handler() -> impl IntoResponse {
    let template = LandingPageTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
