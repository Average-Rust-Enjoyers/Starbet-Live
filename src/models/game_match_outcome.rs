#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, sqlx::Type)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GameMatchOutcome {
    WinA,
    WinB,
    Draw,
}
