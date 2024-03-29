use anyhow::Error;
use axum::{error_handling::HandleErrorLayer, Extension};
use tower::ServiceBuilder;

use crate::{
    auth::{session_store::RedisStore, Auth},
    common::{DbPoolHandler, PoolHandler},
    handlers::error::{handle_timeout_error, handler_404},
    models::extension_web_socket::{ExtensionWebSocketError, ExtensionWebSocketMatch},
    repositories::{
        bet::BetRepository, game::GameRepository, game_match::GameMatchRepository,
        odds::OddsRepository, user::UserRepository,
    },
    routers::{auth_router, protected_router, public_router},
    DbRepository,
};
use axum_login::{login_required, AuthManagerLayerBuilder};
use bb8_redis::RedisConnectionManager;
use sqlx::postgres::PgPoolOptions;
use std::{io, net::SocketAddr, sync::Arc};
use time::Duration;

use tower_sessions::{Expiry, SessionManagerLayer};

pub type RedisPool = bb8::Pool<RedisConnectionManager>;

pub struct App {
    pub pg_pool_handler: PoolHandler,
    pub redis_pool: RedisPool,
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

        let redis_manager = RedisConnectionManager::new(redis_url.clone())?;
        let redis_pool = bb8::Pool::builder().build(redis_manager).await?;

        redis_pool.get().await?; // checks if redis connection is working

        let app = App {
            redis_pool,
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

    pub async fn serve(
        self,
        address: SocketAddr,
        session_expiration: Duration,
    ) -> Result<(), io::Error> {
        let session_store = RedisStore::new(self.redis_pool.clone());
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(session_expiration));

        let user_repo = UserRepository::new(self.pg_pool_handler.clone());
        let game_match_repo = GameMatchRepository::new(self.pg_pool_handler.clone());
        let game_repo = GameRepository::new(self.pg_pool_handler.clone());
        let odds_repo = OddsRepository::new(self.pg_pool_handler.clone());
        let bets_repo = BetRepository::new(self.pg_pool_handler.clone());

        let auth_backend = Auth::new(self.pg_pool_handler);
        let auth_layer = AuthManagerLayerBuilder::new(auth_backend, session_layer).build();

        let (tx_match, rx_match) = barrage::unbounded();
        let web_socket_match = ExtensionWebSocketMatch {
            tx: tx_match,
            rx: rx_match,
        };

        let (tx_error, rx_error) = barrage::unbounded();
        let web_socket_error = ExtensionWebSocketError {
            tx: tx_error,
            rx: rx_error,
        };

        let app = protected_router()
            .route_layer(login_required!(Auth, login_url = "/login"))
            .merge(auth_router())
            .merge(public_router())
            .layer(Extension(user_repo))
            .layer(Extension(bets_repo))
            .layer(Extension(game_match_repo))
            .layer(Extension(game_repo))
            .layer(Extension(odds_repo))
            .layer(Extension(web_socket_match))
            .layer(Extension(web_socket_error.clone()))
            .layer(Extension(self.redis_pool))
            .layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(handle_timeout_error))
                    .timeout(std::time::Duration::from_secs(30)),
            )
            .layer(auth_layer)
            .fallback(handler_404);

        let listener = tokio::net::TcpListener::bind(address).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}
