#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, sqlx::Type)]
pub enum GameMatchOutcome {
    WinA,
    WinB,
    Draw,
}
