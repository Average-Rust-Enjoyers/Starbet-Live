use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, sqlx::Type)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GameMatchOutcome {
    WinA,
    WinB,
    Draw,
}

impl Display for GameMatchOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::WinA => "WIN A",
                Self::WinB => "WIN B",
                Self::Draw => "DRAW",
            }
        )
    }
}
