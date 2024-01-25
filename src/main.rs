#[cfg(debug_assertions)]
use dotenvy::dotenv;

use std::{env, net::SocketAddr};

use starbet_live::app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // run this only in debug mode, docker with release mode has .env variables already set
    #[cfg(debug_assertions)]
    dotenv().expect(".env file not found");

    let pool_connections = 5;

    let port = std::env::var("STARBET_PORT").unwrap_or_else(|_| "6969".to_string());
    let database_url = env::var("DATABASE_URL").expect("missing DATABASE_URL env variable");
    let redis_url = env::var("REDIS_URL").expect("missing REDIS_URL env variable");

    let socket_addr = SocketAddr::from(([0, 0, 0, 0], port.parse::<u16>()?));

    let app = App::config(database_url, redis_url, pool_connections)
        .await?
        .run_migrations()
        .await?;

    println!("Starting server. Listening on http://{socket_addr}");

    app.serve(socket_addr).await?;

    Ok(())
}
