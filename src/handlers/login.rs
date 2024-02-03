use crate::auth::AuthSession;
use crate::error::AppResult;
use crate::models::user::Credentials;
use crate::routers::HxRedirect;
use crate::templates::LoginPage;
use askama::Template;
use axum::{
    http::StatusCode,
    http::Uri,
    response::{Html, IntoResponse, Redirect},
    Form,
};
use std::str::FromStr;

pub mod get {

    use super::*;

    pub async fn login(auth_session: AuthSession) -> AppResult<impl IntoResponse> {
        if auth_session.user.is_some() {
            return Ok(HxRedirect(Uri::from_static("/dashboard")).into_response());
        }
        Ok(Html(LoginPage {}.render()?).into_response())
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
            Ok(None) => {
                return StatusCode::UNAUTHORIZED.into_response(); // authenticate always returns Some
            }
            Err(e) => {
                match e {
                    axum_login::Error::Session(_) => {
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response()
                    }
                    axum_login::Error::Backend(_error_info) => {
                        const LOGIN_URL: &str = "/login";
                        match creds.next {
                            Some(next) => {
                                let l: &str = &format!("{}?next={}", LOGIN_URL, next.clone());
                                match Uri::from_str(l) {
                                    Ok(uri) => {
                                        return HxRedirect(uri).into_response();
                                    }
                                    Err(_) => {
                                        return StatusCode::UNAUTHORIZED.into_response();
                                    }
                                }
                            }
                            None => {
                                return HxRedirect(Uri::from_static(LOGIN_URL)).into_response();
                            }
                        }
                    }
                };
            }
        };

        if auth_session.login(&user).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        HxRedirect(Uri::from_static("/dashboard")).into_response()
    }
}
