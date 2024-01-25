use axum::{
    http,
    routing::{get, post},
    Extension, Router,
};
use bb8_redis::{redis::AsyncCommands, RedisConnectionManager};

use crate::handlers::{
    self, dashboard::dashboard_handler, game::game_handler, index::index_handler,
    register::register_submission_handler,
};

pub fn auth_router() -> Router<()> {
    Router::new()
        .route(
            "/login",
            post(handlers::login::post::login).get(handlers::login::get::login),
        )
        .route(
            "/register",
            post(register_submission_handler).get(handlers::register::register_page_handler),
        )
        .route("/logout", get(handlers::login::get::logout))
        .route(
            "/validation/:field",
            post(handlers::validation::validation_handler),
        )
}

pub fn protected_router() -> Router<()> {
    Router::new()
        .route("/redis", get(redis_ok))
        .route("/dashboard", get(dashboard_handler))
        .route("/games/:name", post(game_handler))
}

pub fn public_router() -> Router<()> {
    Router::new().route("/", get(index_handler))
}

// TODO: remove after first actual handler with redis is implemented
/// # Panics
pub async fn redis_ok(
    Extension(redis_pool): Extension<bb8::Pool<RedisConnectionManager>>,
) -> http::StatusCode {
    let mut conn = redis_pool.get().await.unwrap();
    let value = 42;
    let my_key = "my_key";

    let _: () = conn.set(my_key, value).await.unwrap();
    let return_value: i64 = conn.get(my_key).await.unwrap();
    assert_eq!(value, return_value);
    http::StatusCode::OK
}
