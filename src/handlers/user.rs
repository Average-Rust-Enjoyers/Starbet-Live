use crate::{auth::AuthSession, templates::UserBalance};
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn user_balance_handler(auth_session: AuthSession) -> impl IntoResponse {
    let Some(user) = auth_session.user else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let balance = user.balance;

    let user_balance = UserBalance { balance }.render().unwrap();

    (StatusCode::OK, Html(user_balance)).into_response()
}
