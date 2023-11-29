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
