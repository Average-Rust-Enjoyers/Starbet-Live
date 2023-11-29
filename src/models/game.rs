use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GameGenre {
    MOBA,
    FPS,
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
    pub fn new(name: String, description: String, logo: String, genre: GameGenre) -> Self {
        Self {
            name,
            description,
            logo,
            genre,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub genre: Option<GameGenre>,
}

impl GameUpdate {
    pub fn new(
        name: Option<String>,
        description: Option<String>,
        logo: Option<String>,
        genre: Option<GameGenre>,
    ) -> Self {
        Self {
            name,
            description,
            logo,
            genre,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameDelete {
    pub id: Uuid,
}

impl GameDelete {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}
