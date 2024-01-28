#![allow(dead_code)]

use chrono::{DateTime, Utc};
use float_cmp::approx_eq;
use uuid::Uuid;

#[allow(clippy::struct_field_names)]
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Odds {
    pub id: Uuid,
    // ----------
    pub game_match_id: Uuid,
    // ----------
    pub odds_a: f64,
    pub odds_b: f64,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl PartialEq for Odds {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.game_match_id == other.game_match_id
            && self.created_at == other.created_at
            && self.deleted_at == other.deleted_at
            && approx_eq!(f64, self.odds_a, other.odds_a)
            && approx_eq!(f64, self.odds_b, other.odds_b)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct OddsCreate {
    pub id: Uuid,
    pub game_match_id: Uuid,
    pub odds_a: f64,
    pub odds_b: f64,
}

impl OddsCreate {
    pub fn new(id: &Uuid, game_match_id: &Uuid, odds_a: f64, odds_b: f64) -> Self {
        Self {
            id: id.to_owned(),
            game_match_id: game_match_id.to_owned(),
            odds_a: odds_a.to_owned(),
            odds_b: odds_b.to_owned(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OddsDelete {
    pub id: Uuid,
}

impl OddsDelete {
    pub fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OddsGetById {
    pub id: Uuid,
}

impl OddsGetById {
    pub fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OddsGetByBetId {
    pub bet_id: Uuid,
}

impl OddsGetByBetId {
    pub fn new(bet_id: &Uuid) -> Self {
        Self { bet_id: *bet_id }
    }
}

impl From<&OddsDelete> for OddsGetById {
    fn from(odds_delete_id: &OddsDelete) -> Self {
        Self {
            id: odds_delete_id.id,
        }
    }
}
