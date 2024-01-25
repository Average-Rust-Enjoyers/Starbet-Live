use axum::{
    http,
    routing::{get, post},
    Extension, Router,
};

use bb8_redis::RedisConnectionManager;

#[cfg(debug_assertions)]
use dotenvy::dotenv;

use crate::{
    common::{DbPoolHandler, PoolHandler},
    handlers::{
        bet::{get_bet_handler, place_bet_handler},
        dashboard::dashboard_handler,
        game::game_handler,
        index::index_handler,
        login::login_handler,
        register::{register_page_handler, register_submission_handler},
        validation::validation_handler,
        ws::ws_handler,
    },
};

use models::extension_web_socket::ExtensionWebSocket;
use redis::AsyncCommands;
use sqlx::postgres::PgPoolOptions;
use std::{env, sync::Arc};

mod common;
mod filters;
mod handlers;
mod helpers;
mod models;
mod repositories;
mod templates;
mod validators;

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

    let pool_handler = PoolHandler::new(Arc::new(postgres_pool.clone()));
    let user_repo = repositories::user::UserRepository::new(pool_handler.clone());
    let game_match_repo = repositories::game_match::GameMatchRepository::new(pool_handler.clone());
    let game_repo = repositories::game::GameRepository::new(pool_handler.clone());
    let odds_repo = repositories::odds::OddsRepository::new(pool_handler.clone());
    let (tx, rx) = barrage::unbounded();
    let web_socket = ExtensionWebSocket { tx, rx };

    println!("Starting server. Listening on http://{addr}");

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/login", get(login_handler))
        .route("/register", get(register_page_handler))
        .route("/register", post(register_submission_handler))
        .route("/dashboard", get(dashboard_handler))
        .route("/redis", get(redis_ok))
        .route("/games/:game_id", post(game_handler))
        .route("/validation/:field", post(validation_handler))
        .route("/ws/:game_name", get(ws_handler))
        .route("/bet/:match_id", post(place_bet_handler))
        .route("/bet/:match_id/:prediction", get(get_bet_handler))
        .layer(Extension(user_repo))
        .layer(Extension(game_match_repo))
        .layer(Extension(game_repo))
        .layer(Extension(odds_repo))
        .layer(Extension(redis_pool))
        .layer(Extension(web_socket));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

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
