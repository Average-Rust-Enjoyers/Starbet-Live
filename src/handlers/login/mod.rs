use crate::templates::LoginPageTemplate;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn login_page_handler() -> impl IntoResponse {
    let template = LoginPageTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
