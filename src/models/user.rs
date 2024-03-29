use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum_login::AuthUser;
use chrono::{DateTime, Utc};
use rand::rngs::OsRng;
use serde::Deserialize;
use uuid::Uuid;

use crate::handlers::validation::RegisterFormData;

/// User structure which is serialized from the database, containing full information
/// about the user. Only obtainable when you have the right email and the right password hash
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
    pub balance: i32,
    pub is_admin: bool,
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
}

impl UserCreate {
    pub fn new(
        username: &str,
        email: &str,
        name: &str,
        surname: &str,
        profile_picture: &str,
        password_hash: &str,
    ) -> Self {
        Self {
            username: username.to_owned(),
            email: email.to_owned(),
            name: name.to_owned(),
            surname: surname.to_owned(),
            profile_picture: profile_picture.to_owned(),
            password_hash: password_hash.to_owned(),
        }
    }
}

impl From<RegisterFormData> for UserCreate {
    fn from(register_form_data: RegisterFormData) -> Self {
        Self {
            username: register_form_data.username.clone(),
            email: register_form_data.email,
            name: register_form_data.first_name,
            surname: register_form_data.last_name,
            profile_picture: format!(
                "https://robohash.org/{}.png?set=set2",
                register_form_data.username
            ),
            password_hash: hash_password(register_form_data.password.as_bytes()),
        }
    }
}

/// For verification check example: <https://docs.rs/argon2/latest/argon2>
fn hash_password(password: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password, &salt)
        .expect("Password hashing failed")
        .to_string()
}

/// Structure passed to the repository when trying to log in (read one == login)
#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
    pub next: Option<String>,
}

impl Credentials {
    pub fn new(email: &str, password: &str) -> Self {
        Self {
            email: email.to_owned(),
            password: password.to_owned(),
            next: None,
        }
    }
}

/// Structure passed to the repository when trying to update a user
#[derive(Debug, Clone, Default)]
pub struct UserUpdate {
    pub id: Uuid,
    pub username: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub profile_picture: Option<String>,
    pub password_hash: Option<String>,
    pub balance: Option<i32>,
}

pub struct UserUpdateBalance {
    pub id: Uuid,
    pub delta: i32,
}

impl UserUpdate {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: &Uuid,
        username: Option<&str>,
        email: Option<&str>,
        name: Option<&str>,
        surname: Option<&str>,
        profile_picture: Option<&str>,
        password_hash: Option<&str>,
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
            balance,
        }
    }

    pub fn default(id: &Uuid) -> Self {
        Self {
            id: *id,
            username: None,
            email: None,
            name: None,
            surname: None,
            profile_picture: None,
            password_hash: None,
            balance: None,
        }
    }

    pub const fn update_fields_none(&self) -> bool {
        self.username.is_none()
            && self.email.is_none()
            && self.name.is_none()
            && self.surname.is_none()
            && self.profile_picture.is_none()
            && self.password_hash.is_none()
            && self.balance.is_none()
    }
}

/// Structure passed to the repository when trying to delete a user
#[derive(Debug, Clone)]
pub struct UserDelete {
    pub id: Uuid,
}

impl UserDelete {
    pub const fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

/// Structure passed to the repository when trying to find a user (generic function) for
/// transactions which check whether the specified user exists
#[derive(Debug, Clone)]
pub struct GetByUserId {
    pub id: Uuid,
}

impl GetByUserId {
    pub const fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

impl From<&UserDelete> for GetByUserId {
    fn from(user_delete: &UserDelete) -> Self {
        Self { id: user_delete.id }
    }
}

impl From<&UserUpdate> for GetByUserId {
    fn from(user_update: &UserUpdate) -> Self {
        Self { id: user_update.id }
    }
}

impl AuthUser for User {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password_hash.as_bytes()
    }
}
