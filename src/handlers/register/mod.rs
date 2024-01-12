use crate::templates::{RegisterFormTemplate, RegisterPageTemplate};
use askama::Template;
use axum::{
    extract::Request,
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn register_handler(req: Request) -> impl IntoResponse {
    // If the reqest came from HTMX, render only the form
    // and don't do a full page refresh
    let reply_html = if req.headers().contains_key("referer") {
        (RegisterFormTemplate {}).render().unwrap()
    } else {
        (RegisterPageTemplate {}).render().unwrap()
    };
    (StatusCode::OK, Html(reply_html).into_response())
}
