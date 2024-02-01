#[cfg(debug_assertions)]
use dotenvy::dotenv;
use starbet_live::{
    api::{cloudbet::CloudbetApi, connector::ApiConnector},
    app::App,
    config::{API_POLL_INTERVAL, POOL_CONNECTIONS, SESSION_EXPIRY},
};
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // run this only in debug mode, docker with release mode has .env variables already set
    #[cfg(debug_assertions)]
    dotenv().expect(".env file not found");

    let port = std::env::var("STARBET_PORT").unwrap_or_else(|_| "6969".to_string());
    let database_url = env::var("DATABASE_URL").expect("missing DATABASE_URL env variable");
    let redis_url = env::var("REDIS_URL").expect("missing REDIS_URL env variable");

    let cloudbet_api_key =
        env::var("CLOUDBET_API_KEY").expect("missing CLOUDBET_API_KEY env variable");

    let socket_addr: std::net::SocketAddr = SocketAddr::from(([0, 0, 0, 0], port.parse::<u16>()?));

    let app = App::config(database_url, redis_url, POOL_CONNECTIONS)
        .await?
        .run_migrations()
        .await?;

    let api = ApiConnector::new(app.pg_pool_handler.clone());
    let cloudbet = CloudbetApi::new(cloudbet_api_key);

    println!("Starting server. Listening on http://{socket_addr}");
    // TODO: find out how to handle errors here? is it needed?
    // Expected behavior is to panic on app error, continue (and notify) on api error
    let _ = tokio::join!(
        app.serve(socket_addr, SESSION_EXPIRY),
        api.serve(cloudbet, API_POLL_INTERVAL),
    );

    Ok(())
}
