use crate::templates::RegisterPageTemplate;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn register_page_handler() -> impl IntoResponse {
    let template = RegisterPageTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
