#![allow(dead_code)]

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct Bet {
    pub id: Uuid,
    // ----------
    pub app_user_id: Uuid,
    pub game_match_id: Uuid,
    // ----------
    pub amount: i32,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BetCreate {
    pub id: Uuid,
    pub app_user_id: Uuid,
    pub game_match_id: Uuid,
    pub amount: i32,
}

impl BetCreate {
    pub fn new(id: &Uuid, app_user_id: &Uuid, game_match_id: &Uuid, amount: i32) -> Self {
        Self {
            id: id.to_owned(),
            app_user_id: app_user_id.to_owned(),
            game_match_id: game_match_id.to_owned(),
            amount,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BetUpdate {
    pub id: Uuid,
    pub amount: Option<i32>,
}

impl BetUpdate {
    pub fn new(id: &Uuid, amount: Option<i32>) -> Self {
        Self {
            id: id.to_owned(),
            amount,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BetDelete {
    pub id: Uuid,
}

impl BetDelete {
    pub fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BetGetById {
    pub id: Uuid,
}

impl BetGetById {
    pub fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

impl From<&BetDelete> for BetGetById {
    fn from(bet_delete: &BetDelete) -> Self {
        Self { id: bet_delete.id }
    }
}

impl From<&BetUpdate> for BetGetById {
    fn from(bet_update: &BetUpdate) -> Self {
        Self { id: bet_update.id }
    }
}