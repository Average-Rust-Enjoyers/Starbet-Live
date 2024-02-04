use crate::auth::AuthSession;
use crate::models::user::Credentials;
use crate::routers::HxRedirect;
use crate::{error::AppError, error::AppResult, templates::LoginPage, templates::TextField};
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
        Ok(Html(
            LoginPage {
                email: TextField::new("email"),
            }
            .render()?,
        )
        .into_response())
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
    ) -> AppResult<impl IntoResponse> {
        let user = match auth_session.authenticate(creds.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Err(AppError::StatusCode(StatusCode::UNAUTHORIZED)); // authenticate always returns Some
            }
            Err(e) => {
                match e {
                    axum_login::Error::Session(_) => {
                        return Err(AppError::StatusCode(StatusCode::INTERNAL_SERVER_ERROR));
                    }
                    axum_login::Error::Backend(_error_info) => {
                        const LOGIN_URL: &str = "/login";
                        match creds.next {
                            Some(next) => {
                                let l: &str = &format!("{}?next={}", LOGIN_URL, next.clone());
                                match Uri::from_str(l) {
                                    Ok(uri) => {
                                        return Ok(HxRedirect(uri).into_response());
                                    }
                                    Err(_) => {
                                        return Err(AppError::StatusCode(StatusCode::UNAUTHORIZED));
                                    }
                                }
                            }
                            None => {
                                return Ok(Html(
                                    LoginPage {
                                        email: TextField {
                                            name: "email",
                                            value: creds.email,
                                            error_message: "User not found or password incorrect"
                                                .to_string(),
                                        },
                                    }
                                    .render()?,
                                )
                                .into_response())
                            }
                        }
                    }
                };
            }
        };

        auth_session.login(&user).await?;

        Ok(HxRedirect(Uri::from_static("/dashboard")).into_response())
    }
}
