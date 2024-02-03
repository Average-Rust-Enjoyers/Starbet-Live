use crate::{error::AppError, templates::Index};
use askama::Template;
use axum::response::Html;

pub async fn index_handler() -> Result<Html<String>, AppError> {
    let template = Index {};
    let reply_html = template.render()?;
    Ok(Html(reply_html))
}
