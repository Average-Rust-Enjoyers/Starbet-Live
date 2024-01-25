use anyhow::Error;
use axum::Extension;

use crate::{
    auth::Auth,
    common::{DbPoolHandler, PoolHandler},
    models::extension_web_socket::ExtensionWebSocket,
    repositories::{game_match::GameMatchRepository, odds::OddsRepository, user::UserRepository},
    routers::{auth_router, protected_router, public_router},
    GameRepository,
};
use axum_login::{login_required, AuthManagerLayerBuilder};
use bb8_redis::RedisConnectionManager;
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, sync::Arc};
use time::Duration;
use tower_sessions::{
    fred::{clients::RedisPool, interfaces::ClientLike, types::RedisConfig},
    Expiry, RedisStore, SessionManagerLayer,
};

pub struct App {
    pub pg_pool_handler: PoolHandler,
    pub bb8_redis_pool: bb8::Pool<RedisConnectionManager>,
    pub fred_redis_pool: RedisPool,
}

impl App {
    pub async fn config(
        database_url: String,
        redis_url: String,
        pool_connections: u32,
    ) -> Result<Self, Error> {
        let postgres_pool = PgPoolOptions::new()
            .max_connections(pool_connections)
            .connect(&database_url)
            .await?;
        let pg_pool_handler = PoolHandler::new(Arc::new(postgres_pool.clone()));

        let fred_config = RedisConfig::from_url(&redis_url)?;
        let fred_redis_pool = RedisPool::new(fred_config, None, None, None, 6)?; // TODO: remove constant

        fred_redis_pool.connect();
        fred_redis_pool.wait_for_connect().await?;

        let bb8_manager = RedisConnectionManager::new(redis_url.clone())?;
        let bb8_redis_pool = bb8::Pool::builder().build(bb8_manager).await?;

        let app = App {
            fred_redis_pool,
            bb8_redis_pool,
            pg_pool_handler,
        };
        Ok(app)
    }

    pub async fn run_migrations(self) -> Result<Self, sqlx::Error> {
        sqlx::migrate!("./migrations")
            .run(self.pg_pool_handler.pool.as_ref())
            .await?;
        Ok(self)
    }

    pub async fn serve(self, address: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        let session_store = RedisStore::new(self.fred_redis_pool.clone());
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(Duration::hours(48))); // TODO: remove constant

        let user_repo = UserRepository::new(self.pg_pool_handler.clone());
        let game_match_repo = GameMatchRepository::new(self.pg_pool_handler.clone());
        let game_repo = GameRepository::new(self.pg_pool_handler.clone());
        let odds_repo = OddsRepository::new(self.pg_pool_handler.clone());

        let auth_backend = Auth::new(self.pg_pool_handler);
        let auth_layer = AuthManagerLayerBuilder::new(auth_backend, session_layer).build();

        let (tx, rx) = barrage::unbounded();
        let web_socket = ExtensionWebSocket { tx, rx };

        let app = protected_router()
            .route_layer(login_required!(Auth, login_url = "/login"))
            .merge(auth_router())
            .merge(public_router())
            .layer(auth_layer)
            .layer(Extension(user_repo))
            .layer(Extension(game_match_repo))
            .layer(Extension(game_repo))
            .layer(Extension(odds_repo))
            .layer(Extension(web_socket))
            .layer(Extension(self.bb8_redis_pool));

        let listener = tokio::net::TcpListener::bind(address).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}
