use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

use crate::{
    common::{
        error::{
            BusinessLogicError,
            BusinessLogicErrorKind::{
                UserDeleted, UserDoesNotExist, UserPasswordDoesNotMatch, UserUpdateParametersEmpty,
            },
            DbError, DbResultMultiple, DbResultSingle,
        },
        repository::{
            DbCreate, DbDelete, DbPoolHandler, DbReadOne, DbRepository, DbUpdate, PoolHandler,
        },
    },
    models::user::{GetByUserId, User, UserCreate, UserDelete, UserLogin, UserUpdate, UserUpdateBalance},
};

pub enum Field {
    Username,
    Email,
}

#[derive(Clone)]
pub struct UserRepository {
    pool_handler: PoolHandler,
}

impl UserRepository {
    pub fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    /// Function which retrieves a user by their id, usable within a transaction
    ///
    /// # Params
    /// - `params`: structure containing the id of the user
    /// - `transaction_handle` mutable reference to an ongoing transaction
    ///
    /// # Returns
    /// - `Ok(user)`: on successful connection and retrieval
    /// - `Err(_)`: otherwise
    ///
    /// # Panics
    /// # Errors
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
    ///
    /// # Errors
    pub fn is_correct(user: Option<User>) -> DbResultSingle<User> {
        match user {
            Some(user) if user.deleted_at.is_some() => {
                Err(BusinessLogicError::new(UserDeleted).into())
            }
            Some(user) => Ok(user),
            None => Err(BusinessLogicError::new(UserDoesNotExist).into()),
        }
    }

    /// Function which checks if the field is in use
    ///
    /// # Params
    /// - `field`: field to check
    /// - `value`: value to check
    ///
    /// # Returns
    /// - `Ok(true)`: if the field is in use
    /// - `Ok(false)`: if the field is not in use
    /// - `Err(_)`: otherwise
    ///
    /// # Errors
    /// - `sqlx::Error`: if the query fails
    pub async fn is_field_in_use(
        &mut self,
        field: Field,
        value: &str,
    ) -> Result<bool, sqlx::Error> {
        let query = match field {
            Field::Username => {
                r"
                SELECT 1
                FROM AppUser
                WHERE username=$1
            "
            }
            Field::Email => {
                r"
                SELECT 1
                FROM AppUser
                WHERE email=$1
            "
            }
        };

        let user = sqlx::query(query)
            .bind(value)
            .fetch_optional(self.pool_handler.pool.as_ref())
            .await?;

        Ok(user.is_some())
    }

    pub async fn update_user_balance<'a>(
        params: UserUpdateBalance,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<User> {

        let user = Self::is_correct(
            Self::get_user(GetByUserId { id: params.id }, &mut *transaction_handle).await?
        )?;
        
        let user = sqlx::query_as!(
            User,
            r#"
                UPDATE AppUser
                SET edited_at = NOW(), 
                    balance = $1
                WHERE id = $2
                RETURNING *
            "#,
            user.balance + params.delta,
            params.id
        )
        .fetch_one(transaction_handle.as_mut())
        .await?;

        Ok(user)
    }
}

#[async_trait]
impl DbRepository for UserRepository {
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

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
                    profile_picture, password_hash)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING *
            "#,
            data.username,
            data.email,
            data.name,
            data.surname,
            data.profile_picture,
            data.password_hash,
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

        if let Ok(parsed_hash) = PasswordHash::new(&user.password_hash) {
            let check_result =
                Argon2::default().verify_password(params.password.as_bytes(), &parsed_hash);
            if check_result.is_err() {
                return Err(BusinessLogicError::new(UserPasswordDoesNotMatch).into());
            }
        } else {
            return Err(DbError::new("invalid hash"));
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
                    balance = COALESCE($7, balance)
                WHERE id=$8
                RETURNING *
            "#,
            params.username,
            params.email,
            params.name,
            params.surname,
            params.profile_picture,
            params.password_hash,
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
