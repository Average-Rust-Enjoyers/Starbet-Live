use std::sync::Arc;

use axum::extract::FromRef;
use bb8_redis::RedisConnectionManager;

use crate::repositories::user::UserRepository;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub postgres_pool: sqlx::PgPool,
    pub redis_pool: bb8::Pool<RedisConnectionManager>,
    pub user_repo: Arc<UserRepository>,
}
