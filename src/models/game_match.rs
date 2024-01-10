#![allow(dead_code)]

use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::game_match_outcome::GameMatchOutcome;

#[derive(Debug, PartialEq, Eq, Clone, sqlx::Type)]
pub enum GameMatchStatus {
    Pending,
    Live,
    Finished,
    Canceled,
}

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct GameMatch {
    pub id: Uuid,
    // ----------
    pub game_id: Uuid,
    // ----------
    pub name_a: String,
    pub name_b: String,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub outcome: Option<GameMatchOutcome>,
    pub status: GameMatchStatus,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameMatchCreate {
    pub game_id: Uuid,
    pub name_a: String,
    pub name_b: String,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
}

impl GameMatchCreate {
    pub fn new(
        game_id: &Uuid,
        name_a: &str,
        name_b: &str,
        starts_at: DateTime<Utc>,
        ends_at: DateTime<Utc>,
    ) -> Self {
        Self {
            game_id: game_id.to_owned(),
            name_a: name_a.to_owned(),
            name_b: name_b.to_owned(),
            starts_at,
            ends_at,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameMatchUpdate {
    pub id: Uuid,
    pub name_a: Option<String>,
    pub name_b: Option<String>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub status: Option<GameMatchStatus>,
}

impl GameMatchUpdate {
    pub fn new(
        id: &Uuid,
        name_a: Option<&str>,
        name_b: Option<&str>,
        starts_at: Option<DateTime<Utc>>,
        ends_at: Option<DateTime<Utc>>,
        status: Option<GameMatchStatus>,
    ) -> Self {
        let change_to_owned = |value: &str| Some(value.to_owned());
        Self {
            id: id.to_owned(),
            name_a: name_a.and_then(change_to_owned),
            name_b: name_b.and_then(change_to_owned),
            starts_at,
            ends_at,
            status,
        }
    }

    pub const fn update_fields_none(&self) -> bool {
        self.name_a.is_none()
            && self.name_b.is_none()
            && self.starts_at.is_none()
            && self.ends_at.is_none()
            && self.status.is_none()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameMatchDelete {
    pub id: Uuid,
}

impl GameMatchDelete {
    pub fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameMatchGetById {
    pub id: Uuid,
}

impl GameMatchGetById {
    pub fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameMatchSetOutcome {
    pub id: Uuid,
    pub outcome: GameMatchOutcome,
}

impl GameMatchSetOutcome {
    pub fn new(id: &Uuid, outcome: GameMatchOutcome) -> Self {
        Self {
            id: id.to_owned(),
            outcome,
        }
    }
}

impl From<&GameMatchDelete> for GameMatchGetById {
    fn from(game_match_delete: &GameMatchDelete) -> Self {
        Self {
            id: game_match_delete.id,
        }
    }
}

impl From<&GameMatchUpdate> for GameMatchGetById {
    fn from(game_match_update: &GameMatchUpdate) -> Self {
        Self {
            id: game_match_update.id,
        }
    }
}
