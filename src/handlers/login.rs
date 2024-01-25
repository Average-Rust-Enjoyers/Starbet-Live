use crate::auth::AuthSession;
use crate::models::user::Credentials;
use crate::templates::LoginPage;
use askama::Template;
use axum::Form;
use axum::{
    body::Body,
    http::{HeaderValue, Response},
};
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};

pub mod get {

    use super::*;

    pub async fn login(auth_session: AuthSession) -> impl IntoResponse {
        if auth_session.user.is_some() {
            return Redirect::to("/dashboard").into_response();
        }
        Html(LoginPage {}.render().unwrap()).into_response()
    }

    pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.logout().await {
            Ok(_) => Redirect::to("/").into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

pub mod post {

    use super::*;

    pub async fn login(
        mut auth_session: AuthSession,
        Form(creds): Form<Credentials>,
    ) -> impl IntoResponse {
        let user = match auth_session.authenticate(creds.clone()).await {
            Ok(Some(user)) => user,
            // TODO: this can not happen at the moment because we always send Some()
            Ok(None) => {
                let mut login_url = "/login".to_string();
                if let Some(next) = creds.next {
                    login_url = format!("{}?next={}", login_url, next.clone());
                };
                let hx_redirect_value = HeaderValue::from_str(login_url.as_str()).unwrap();

                // TODO: fix temporary ugly responses
                let response = Response::builder()
                    .status(StatusCode::OK) // TODO: change to correct status code
                    .header("HX-Redirect", hx_redirect_value)
                    .body(Body::from("redirecting..."))
                    .unwrap();
                return response;
            }
            Err(_) => {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };

        if auth_session.login(&user).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        let hx_redirect_value = match creds.next {
            // TODO: redirect to "next" value is not really working
            Some(ref next) => HeaderValue::from_str(next.as_str()).unwrap(),
            None => HeaderValue::from_static("/dashboard"),
        };

        // TODO: fix temporary ugly responses
        Response::builder()
            .status(StatusCode::OK) // TODO: change to correct status code
            .header("HX-Redirect", hx_redirect_value)
            .body(Body::from("redirecting..."))
            .unwrap()
    }
}
