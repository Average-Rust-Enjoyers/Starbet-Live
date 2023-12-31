#![allow(dead_code)]

use chrono::{DateTime, Utc};
use uuid::Uuid;

/// User structure which is serialized from the database, containing full information
/// about the user. Only obtainable when you have the right email and the right password hash
/// (auth is not the scope of this iteration, this would be done way differently if it was).
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    // --------------
    pub username: String,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub profile_picture: String,
    pub password_hash: String,
    pub password_salt: String,
    pub balance: i32,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Structure passed to the repository for User creation
#[derive(Debug, Clone)]
pub struct UserCreate {
    pub username: String,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub profile_picture: String,
    pub password_hash: String,
    pub password_salt: String,
}

impl UserCreate {
    #[must_use]
    pub fn new(
        username: &str,
        email: &str,
        name: &str,
        surname: &str,
        profile_picture: &str,
        password_hash: &str,
        password_salt: &str,
    ) -> Self {
        Self {
            username: username.to_owned(),
            email: email.to_owned(),
            name: name.to_owned(),
            surname: surname.to_owned(),
            profile_picture: profile_picture.to_owned(),
            password_hash: password_hash.to_owned(),
            password_salt: password_salt.to_owned(),
        }
    }
}

/// Structure passed to the repository when trying to log in (read one == login)
#[derive(Debug, Clone)]
pub struct UserLogin {
    pub email: String,
    pub password_hash: String,
}

impl UserLogin {
    #[must_use]
    pub fn new(email: &str, password_hash: &str) -> Self {
        Self {
            email: email.to_owned(),
            password_hash: password_hash.to_owned(),
        }
    }
}

/// Structure passed to the repository when trying to update a user
#[derive(Debug, Clone)]
pub struct UserUpdate {
    pub id: Uuid,
    pub username: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub profile_picture: Option<String>,
    pub password_hash: Option<String>,
    pub password_salt: Option<String>,
    pub balance: Option<i32>,
}

impl UserUpdate {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: &Uuid,
        username: Option<&str>,
        email: Option<&str>,
        name: Option<&str>,
        surname: Option<&str>,
        profile_picture: Option<&str>,
        password_hash: Option<&str>,
        password_salt: Option<&str>,
        balance: Option<i32>,
    ) -> Self {
        let change_to_owned = |value: &str| Some(value.to_owned());
        Self {
            id: *id,
            username: username.and_then(change_to_owned),
            email: email.and_then(change_to_owned),
            name: name.and_then(change_to_owned),
            surname: surname.and_then(change_to_owned),
            profile_picture: profile_picture.and_then(change_to_owned),
            password_hash: password_hash.and_then(change_to_owned),
            password_salt: password_salt.and_then(change_to_owned),
            balance,
        }
    }

    #[must_use]
    pub const fn update_fields_none(&self) -> bool {
        self.username.is_none()
            && self.email.is_none()
            && self.name.is_none()
            && self.surname.is_none()
            && self.profile_picture.is_none()
            && self.password_hash.is_none()
            && self.password_salt.is_none()
            && self.balance.is_none()
    }
}

/// Structure passed to the repository when trying to delete a user
#[derive(Debug, Clone)]
pub struct UserDelete {
    pub id: Uuid,
}

impl UserDelete {
    #[must_use]
    pub const fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

/// Structure passed to the repository when trying to find a user (generic function) for
/// transactions which check whether the specified user exists
#[derive(Debug, Clone)]
pub struct UserGetById {
    pub id: Uuid,
}

impl UserGetById {
    #[must_use]
    pub const fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

impl From<&UserDelete> for UserGetById {
    fn from(user_delete: &UserDelete) -> Self {
        Self { id: user_delete.id }
    }
}

impl From<&UserUpdate> for UserGetById {
    fn from(user_update: &UserUpdate) -> Self {
        Self { id: user_update.id }
    }
}
