use axum::{http, routing::get, Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::{arch::x86_64::_mm_dpbf16_ps, env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env file not found");

    let pool_connections = 5;
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let database_url = env::var("DATABASE_URL").expect("missing DATABASE_URL env");

    let pool = PgPoolOptions::new()
        .max_connections(pool_connections)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new().route("/", get(ok)).with_state(pool);
    dbg!(addr.as_str());
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

pub async fn ok() -> http::StatusCode {
    http::StatusCode::OK
}
