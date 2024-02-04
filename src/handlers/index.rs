use crate::{error::AppResult, templates::Index};
use askama::Template;
use axum::response::Html;

pub async fn index_handler() -> AppResult<Html<String>> {
    let template = Index {};
    let reply_html = template.render()?;
    Ok(Html(reply_html))
}
