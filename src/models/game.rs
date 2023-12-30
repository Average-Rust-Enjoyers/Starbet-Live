#![allow(dead_code)]

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GameGenre {
    Moba,
    Fps,
}

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct Game {
    pub id: Uuid,
    // ----------
    pub name: String,
    pub description: String,
    pub logo: String,
    pub genre: GameGenre,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameCreate {
    pub name: String,
    pub description: String,
    pub logo: String,
    pub genre: GameGenre,
}

impl GameCreate {
    pub fn new(name: &str, description: &str, logo: &str, genre: GameGenre) -> Self {
        Self {
            name: name.to_owned(),
            description: description.to_owned(),
            logo: logo.to_owned(),
            genre,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameUpdate {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub genre: Option<GameGenre>,
}

impl GameUpdate {
    pub fn new(
        id: &Uuid,
        name: Option<&str>,
        description: Option<&str>,
        logo: Option<&str>,
        genre: Option<GameGenre>,
    ) -> Self {
        let change_to_owned = |value: &str| Some(value.to_owned());
        Self {
            id: *id,
            name: name.and_then(change_to_owned),
            description: description.and_then(change_to_owned),
            logo: logo.and_then(change_to_owned),
            genre,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameFilter {
    pub name: Option<String>,
    pub genre: Option<GameGenre>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameDelete {
    pub id: Uuid,
}

impl GameDelete {
    pub fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameGetById {
    pub id: Uuid,
}

impl GameGetById {
    pub fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

impl From<&GameDelete> for GameGetById {
    fn from(game_delete: &GameDelete) -> Self {
        Self { id: game_delete.id }
    }
}

impl From<&GameUpdate> for GameGetById {
    fn from(game_update: &GameUpdate) -> Self {
        Self { id: game_update.id }
    }
}
