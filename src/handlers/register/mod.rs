use crate::templates::{RegisterForm, RegisterPage, TextField};
use askama::Template;
use axum::{
    extract::Request,
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn register_handler(req: Request) -> impl IntoResponse {
    let fields = [
        "username",
        "first_name",
        "last_name",
        "email",
        "password",
        "password_confirm",
    ];

    let form = RegisterForm {
        username: TextField::new(fields[0]),
        first_name: TextField::new(fields[1]),
        last_name: TextField::new(fields[2]),
        email: TextField::new(fields[3]),
        password: TextField::new(fields[4]),
        password_confirm: TextField::new(fields[5]),
    };

    // If the reqest came from HTMX, render only the form
    // and don't do a full page refresh
    let reply_html = if req.headers().contains_key("referer") {
        form.render().unwrap()
    } else {
        RegisterPage { form }.render().unwrap()
    };
    (StatusCode::OK, Html(reply_html).into_response())
}
