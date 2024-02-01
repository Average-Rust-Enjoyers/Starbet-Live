use crate::auth::AuthSession;
use crate::models::user::Credentials;
use crate::routers::HxRedirect;
use crate::templates::LoginPage;
use askama::Template;
use axum::http::Uri;
use axum::Form;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use std::str::FromStr;

pub mod get {

    use super::*;

    pub async fn login(auth_session: AuthSession) -> impl IntoResponse {
        if auth_session.user.is_some() {
            return Redirect::to("/dashboard").into_response(); // TODO: do we need hx redirect here?
        }
        Html(LoginPage {}.render().unwrap()).into_response()
    }

    pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.logout().await {
            Ok(_) => Redirect::to("/").into_response(), // TODO: do we need hx redirect here?
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
                const LOGIN_URL: &str = "/login";
                match creds.next {
                    Some(next) => {
                        let l: &str = &format!("{}?next={}", LOGIN_URL, next.clone());
                        match Uri::from_str(l) {
                            Ok(uri) => {
                                return HxRedirect(uri).into_response();
                            }
                            Err(_) => {
                                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                            }
                        }
                    }
                    None => {
                        return HxRedirect(Uri::from_static(LOGIN_URL)).into_response();
                    }
                }
            }
            Err(_) => {
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };

        if auth_session.login(&user).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        HxRedirect(Uri::from_static("/dashboard")).into_response()
    }
}
