use crate::{
    auth::{self, AuthSession},
    error::AppError,
    templates::UserBalance,
};
use askama::Template;
use axum::response::Html;

pub async fn user_balance_handler(auth_session: AuthSession) -> Result<Html<String>, AppError> {
    let user = auth::is_logged_in(auth_session)?;

    let balance = user.balance;

    let user_balance = UserBalance { balance }.render()?;

    Ok(Html(user_balance))
}
