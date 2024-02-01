use crate::{auth, templates::ProfilePage};
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn profile_handler(auth_session: auth::AuthSession) -> impl IntoResponse {
    let Some(user) = auth_session.user else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    (
        StatusCode::OK,
        Html(ProfilePage::from(user).render().unwrap()),
    )
        .into_response()
}
