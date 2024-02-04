use axum::{
    http::{Method, StatusCode, Uri},
    response::IntoResponse,
    BoxError,
};

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "You are lost, go back to safety!").into_response()
}

pub async fn handle_timeout_error(method: Method, uri: Uri, err: BoxError) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("`{method} {uri}` failed with {err}"),
    )
}
