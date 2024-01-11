use crate::templates::LoginPageTemplate;
use askama::Template;
use axum::{
    extract::Request,
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn login_page_handler(req: Request) -> impl IntoResponse {
    if !req.headers().contains_key("referer") {
        return (StatusCode::FOUND, Html("Already logged in").into_response());
    }
    let template = LoginPageTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
