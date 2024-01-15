use axum::{
    extract::{FromRef, State},
    http,
    routing::{get, post},
    Router,
};

use bb8_redis::RedisConnectionManager;

#[cfg(debug_assertions)]
use dotenvy::dotenv;

use crate::handlers::{
    dashboard::dashboard_handler, game::game_handler, index::index_handler, login::login_handler,
    register::register_page_handler, validation::validation_handler,
};

use redis::AsyncCommands;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod common;
mod filters;
mod handlers;
mod models;
mod repositories;
mod templates;
mod validators;

#[derive(FromRef, Clone)]
pub struct AppState {
    postgres_pool: sqlx::PgPool,
    redis_pool: bb8::Pool<RedisConnectionManager>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // run this only in debug mode, docker with release mode has .env variables already set
    #[cfg(debug_assertions)]
    dotenv().expect(".env file not found");

    let pool_connections = 5;
    let port = std::env::var("STARBET_PORT").unwrap_or_else(|_| "6969".to_string());
    let addr = format!("0.0.0.0:{port}");

    let database_url = env::var("DATABASE_URL").expect("missing DATABASE_URL env variable");
    let postgres_pool = PgPoolOptions::new()
        .max_connections(pool_connections)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&postgres_pool).await?;

    let redis_url = env::var("REDIS_URL").expect("missing REDIS_URL env variable");
    let manager = RedisConnectionManager::new(redis_url.clone())?;
    let redis_pool = bb8::Pool::builder().build(manager).await?;

    redis::Client::open(redis_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis");

    let app_state: AppState = AppState {
        postgres_pool,
        redis_pool,
    };

    println!("Starting server. Listening on http://{addr}");

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/login", get(login_handler))
        .route("/register", get(register_page_handler))
        .route("/dashboard", get(dashboard_handler))
        .route("/redis", get(redis_ok))
        .route("/game", post(game_handler))
        .route("/validation/:field", post(validation_handler))
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

/// # Panics
pub async fn redis_ok(
    State(redis_pool): State<bb8::Pool<RedisConnectionManager>>,
) -> http::StatusCode {
    let mut conn = redis_pool.get().await.unwrap();
    let value = 42;
    let my_key = "my_key";

    let _: () = conn.set(my_key, value).await.unwrap();
    let return_value: i64 = conn.get(my_key).await.unwrap();
    assert_eq!(value, return_value);
    http::StatusCode::OK
}
