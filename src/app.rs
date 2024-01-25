use anyhow::Error;
use axum::Extension;

use axum_login::{login_required, AuthManagerLayerBuilder};
use bb8_redis::RedisConnectionManager;
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, sync::Arc};
use time::Duration;

use tower_sessions::{Expiry, SessionManagerLayer};

use crate::{
    auth::Auth,
    common::{DbPoolHandler, PoolHandler},
    repositories::user::UserRepository,
    routers::{auth_router, protected_router, public_router},
    session_store::RedisStore,
};

pub struct App {
    pub pg_pool_handler: PoolHandler,
    pub redis_pool: bb8::Pool<RedisConnectionManager>,
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
    ) -> Result<(), Box<dyn std::error::Error>> {
        let session_store = RedisStore::new(self.redis_pool.clone());
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(session_expiration));

        let user_repo = UserRepository::new(self.pg_pool_handler.clone());

        let auth_backend = Auth::new(self.pg_pool_handler);
        let auth_layer = AuthManagerLayerBuilder::new(auth_backend, session_layer).build();

        let app = protected_router()
            .route_layer(login_required!(Auth, login_url = "/login"))
            .merge(auth_router())
            .merge(public_router())
            .layer(auth_layer)
            .layer(Extension(user_repo))
            .layer(Extension(self.redis_pool));

        let listener = tokio::net::TcpListener::bind(address).await.unwrap();
        axum::serve(listener, app).await.unwrap();

        Ok(())
    }
}
