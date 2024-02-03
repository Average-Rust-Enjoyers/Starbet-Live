use async_trait::async_trait;
use axum_login::{AuthnBackend, UserId};
use sqlx::{Postgres, Transaction};

use crate::common::repository::DbReadOne;
use crate::common::PoolHandler;
use crate::error::{AppError, AppResult};
use crate::models::user::User;
use crate::repositories::user::UserRepository;
use crate::{
    common::error::DbError,
    models::{
        self,
        user::{Credentials, GetByUserId},
    },
};

pub mod session_store;
pub type AuthSession = axum_login::AuthSession<Auth>;

#[derive(Clone)]
pub struct Auth {
    pool_handler: PoolHandler,
}

impl Auth {
    pub fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }
}

#[async_trait]
impl AuthnBackend for Auth {
    type User = models::user::User;
    type Error = DbError;
    type Credentials = Credentials;

    async fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = UserRepository::new(self.pool_handler.clone())
            .read_one(&credentials)
            .await?;
        Ok(Some(user)) // Always returning Some. If user is not found UserRepository.read_one returns DbError
    }

    async fn get_user(&self, id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let mut tx: Transaction<'_, Postgres> = self.pool_handler.pool.begin().await?;
        let user = UserRepository::get_user(GetByUserId { id: *id }, &mut tx).await;
        tx.commit().await?;

        user
    }
}

pub fn is_logged_in(auth_session: AuthSession) -> AppResult<User> {
    match auth_session.user {
        Some(user) => Ok(user),
        None => Err(AppError::ForbiddenError),
    }
}
pub fn is_logged_admin(auth_session: AuthSession) -> AppResult<User> {
    let user = is_logged_in(auth_session)?;
    match user.is_admin {
        true => Ok(user),
        false => Err(AppError::ForbiddenError),
    }
}
