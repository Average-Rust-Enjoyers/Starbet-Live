use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

use crate::{
    common::{
        error::{
            BusinessLogicError,
            BusinessLogicErrorKind::{
                UserDeleted, UserDoesNotExist, UserPasswordDoesNotMatch, UserUpdateParametersEmpty,
            },
            DbResultMultiple, DbResultSingle,
        },
        repository::{
            DbCreate, DbDelete, DbPoolHandler, DbReadOne, DbRepository, DbUpdate, PoolHandler,
        },
    },
    models::user::{GetByUserId, User, UserCreate, UserDelete, UserLogin, UserUpdate},
};

pub struct UserRepository {
    pool_handler: PoolHandler,
}

impl UserRepository {
    /// Function which retrieves a user by their id, usable within a transaction
    ///
    /// # Params
    /// - `params`: structure containing the id of the user
    /// - `transaction_handle` mutable reference to an ongoing transaction
    ///
    /// # Returns
    /// - `Ok(user)`: on successful connection and retrieval
    /// - `Err(_)`: otherwise
    pub async fn get_user<'a>(
        params: GetByUserId,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
                SELECT *
                FROM AppUser
                WHERE id=$1
            "#,
            params.id
        )
        .fetch_optional(&mut **transaction_handle)
        .await?;

        Ok(user)
    }

    /// Function which checks if the user is correct (existing and not deleted)
    ///
    /// # Params
    /// - `user`: optional user retrieved from the database
    ///
    /// # Returns
    /// - `Ok(user)`: when the user exists and is not deleted
    /// - `Err(DbError)`: with appropriate error description otherwise
    pub fn is_correct(user: Option<User>) -> DbResultSingle<User> {
        match user {
            Some(user) if user.deleted_at.is_some() => {
                Err(BusinessLogicError::new(UserDeleted).into())
            }
            Some(user) => Ok(user),
            None => Err(BusinessLogicError::new(UserDoesNotExist).into()),
        }
    }
}

#[async_trait]
impl DbRepository for UserRepository {
    #[inline]
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    #[inline]
    async fn disconnect(&mut self) -> () {
        self.pool_handler.disconnect().await;
    }
}

#[async_trait]
impl DbCreate<UserCreate, User> for UserRepository {
    /// Create a new user with the specified data
    async fn create(&mut self, data: &UserCreate) -> DbResultSingle<User> {
        let user = sqlx::query_as!(
            User,
            r#"
                INSERT INTO AppUser (username, email, name, surname,
                    profile_picture, password_hash, password_salt)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING *
            "#,
            data.username,
            data.email,
            data.name,
            data.surname,
            data.profile_picture,
            data.password_hash,
            data.password_salt
        )
        .fetch_one(self.pool_handler.pool.as_ref())
        .await?;

        Ok(user)
    }
}

#[async_trait]
impl DbReadOne<UserLogin, User> for UserRepository {
    /// Login the user with provided parameters,
    async fn read_one(&mut self, params: &UserLogin) -> DbResultSingle<User> {
        let user = sqlx::query_as!(
            User,
            r#"
                SELECT *
                FROM AppUser
                WHERE email=$1
            "#,
            params.email
        )
        .fetch_optional(self.pool_handler.pool.as_ref())
        .await?;
        let user = Self::is_correct(user)?;

        if user.password_hash != params.password_hash {
            return Err(BusinessLogicError::new(UserPasswordDoesNotMatch).into());
        }

        Ok(user)
    }
}

#[async_trait]
impl DbUpdate<UserUpdate, User> for UserRepository {
    /// Update user information if we know their id (we're logged in as that user)
    /// Fails if the relevant update fields are all none
    async fn update(&mut self, params: &UserUpdate) -> DbResultMultiple<User> {
        if params.update_fields_none() {
            return Err(BusinessLogicError::new(UserUpdateParametersEmpty).into());
        }

        let mut tx = self.pool_handler.pool.begin().await?;
        Self::is_correct(Self::get_user(params.into(), &mut tx).await?)?;

        let user = sqlx::query_as!(
            User,
            r#"
                UPDATE AppUser
                SET edited_at=NOW(), 
                    username = COALESCE($1, username), 
                    email = COALESCE($2, email), 
                    name = COALESCE($3, name), 
                    surname = COALESCE($4, surname), 
                    profile_picture = COALESCE($5, profile_picture), 
                    password_hash = COALESCE($6, password_hash), 
                    password_salt = COALESCE($7, password_salt),
                    balance = COALESCE($8, balance)
                WHERE id=$9
                RETURNING *
            "#,
            params.username,
            params.email,
            params.name,
            params.surname,
            params.profile_picture,
            params.password_hash,
            params.password_salt,
            params.balance,
            params.id
        )
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(user)
    }
}

#[async_trait]
impl DbDelete<UserDelete, User> for UserRepository {
    /// Delete the user if we know their id (we're logged in as that user)
    async fn delete(&mut self, params: &UserDelete) -> DbResultMultiple<User> {
        let mut tx = self.pool_handler.pool.begin().await?;
        Self::is_correct(Self::get_user(params.into(), &mut tx).await?)?;

        let user = sqlx::query_as!(
            User,
            r#"
                UPDATE AppUser
                SET deleted_at=NOW(), edited_at=NOW(), username=$1, email=$1
                WHERE id=$1
                RETURNING *
            "#,
            params.id
        )
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(user)
    }
}
