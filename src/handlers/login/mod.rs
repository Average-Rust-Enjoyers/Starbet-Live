use crate::templates::{LoginFormTemplate, LoginPageTemplate};
use askama::Template;
use axum::{
    extract::Request,
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn login_handler(req: Request) -> impl IntoResponse {
    let reply_html = if req.headers().contains_key("referer") {
        (LoginFormTemplate {}).render().unwrap()
    } else {
        (LoginPageTemplate {}).render().unwrap()
    };
    (StatusCode::OK, Html(reply_html).into_response())
}
