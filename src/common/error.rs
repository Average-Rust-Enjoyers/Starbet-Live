#![allow(dead_code)]

use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum BusinessLogicErrorKind {
    // User errors
    UserDoesNotExist,
    UserDeleted,
    UserPasswordDoesNotMatch,
    // Game errors
    GameDoesNotExist,
    GameDeleted,
    // GameMatch errors
    GameMatchDoesNotExist,
    GameMatchDeleted,
    GameMatchAlreadyStarted,
    GameMatchAlreadyFinished,
    GameMatchStartsAfterEnds,
    // Bet errors
    BetDoesNotExist,
    BetDeleted,
    BetAmountTooLow,
    BetAmountTooHigh,
    // BetAmountNotAllowedMultiple, // TODO: consult this
    // BetAmountNotAllowedZero,
    // BetAmountNotAllowedNegative,
    // MoneyTransaction errors
    MoneyTransactionDoesNotExist,
    MoneyTransactionDeleted,
    MoneyTransactionAmountTooLow,
    MoneyTransactionAmountTooHigh,
    // Odds errors
    OddsDoesNotExist,
    OddsDeleted,

    // --------------------------
    // Generic errors
    UserUpdateParametersEmpty,
}

impl Display for BusinessLogicErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let does_not_exist = |name: &str| format!("The specified {name} does not exist!");
        let deleted = |name: &str| format!("The specified {name} has been deleted!");
        let ammount_too_low = |name: &str| format!("The specified {name} ammount is too low!");
        let ammount_too_high = |name: &str| format!("The specified {name} ammount is too high!");

        let error_string = match self {
            BusinessLogicErrorKind::UserDoesNotExist => does_not_exist("user"),
            BusinessLogicErrorKind::UserDeleted => deleted("user"),
            BusinessLogicErrorKind::UserPasswordDoesNotMatch => {
                "The provided email and password combination is incorrect.".to_string()
            }
            BusinessLogicErrorKind::UserUpdateParametersEmpty => concat!(
                "The provided parameters for User update query are incorrect",
                " (no User field would be changed)."
            )
            .to_string(),
            BusinessLogicErrorKind::GameDoesNotExist => does_not_exist("game"),
            BusinessLogicErrorKind::GameDeleted => deleted("game"),
            BusinessLogicErrorKind::GameMatchDoesNotExist => does_not_exist("game match"),
            BusinessLogicErrorKind::GameMatchDeleted => deleted("game match"),
            BusinessLogicErrorKind::GameMatchAlreadyStarted => {
                "The game match has already started!".to_string()
            }
            BusinessLogicErrorKind::GameMatchAlreadyFinished => {
                "The game match has already finished!".to_string()
            }
            BusinessLogicErrorKind::GameMatchStartsAfterEnds => {
                "The game match starting time is later than the ending time!".to_string()
            }
            BusinessLogicErrorKind::BetDoesNotExist => does_not_exist("bet"),
            BusinessLogicErrorKind::BetDeleted => deleted("bet"),
            BusinessLogicErrorKind::BetAmountTooLow => ammount_too_low("bet"),
            BusinessLogicErrorKind::BetAmountTooHigh => ammount_too_high("bet"),
            // BusinessLogicErrorKind::BetAmountNotAllowedMultiple => todo!(),
            // BusinessLogicErrorKind::BetAmountNotAllowedZero => todo!(),
            // BusinessLogicErrorKind::BetAmountNotAllowedNegative => todo!(),
            BusinessLogicErrorKind::MoneyTransactionDoesNotExist => {
                does_not_exist("money transaction")
            }
            BusinessLogicErrorKind::MoneyTransactionDeleted => deleted("money transaction"),
            BusinessLogicErrorKind::MoneyTransactionAmountTooLow => {
                ammount_too_low("money transaction")
            }
            BusinessLogicErrorKind::MoneyTransactionAmountTooHigh => {
                ammount_too_high("money transaction")
            }
            BusinessLogicErrorKind::OddsDoesNotExist => does_not_exist("odds"),
            BusinessLogicErrorKind::OddsDeleted => deleted("odds"),
        };
        f.write_str(error_string.as_str())
    }
}

/// Error type representing a Business Logic Error in the database layer ->
/// usually a problem with missing records, insufficient rights for operation, and so on.
pub struct BusinessLogicError {
    error: BusinessLogicErrorKind,
}

impl BusinessLogicError {
    /// Business Logic Error constructor
    #[must_use]
    #[inline]
    pub const fn new(error: BusinessLogicErrorKind) -> Self {
        Self { error }
    }

    /// Formatted business logic error
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Business logic error: {}", self.error)
    }
}

impl Display for BusinessLogicError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

impl Debug for BusinessLogicError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

pub struct DbError {
    description: String,
}

/// Error encapsulating errors from `sqlx` and our own `BusinessLogicError`, unifying errors from
/// the database without the need of `anyhow` library.
impl DbError {
    /// Database Error constructor
    #[must_use]
    #[inline]
    pub fn new(description: &str) -> Self {
        Self {
            description: description.to_owned(),
        }
    }
    /// Formatted database error
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Database Error] {}", self.description)
    }
}

impl Display for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

impl Debug for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

/// The database error can be assigned to `dyn Error`
impl std::error::Error for DbError {
    fn description(&self) -> &str {
        &self.description
    }
}

/// Conversion from sqlx error, useful when using `?` operator
impl From<sqlx::Error> for DbError {
    fn from(value: sqlx::Error) -> Self {
        Self::new(&format!("sqlx error: {value}"))
    }
}

/// Conversion from sqlx migrate error, useful when using `?` operator
impl From<sqlx::migrate::MigrateError> for DbError {
    fn from(value: sqlx::migrate::MigrateError) -> Self {
        Self::new(&format!("Migration error: {value}"))
    }
}

/// Conversion from business logic error
impl From<BusinessLogicError> for DbError {
    fn from(value: BusinessLogicError) -> Self {
        Self::new(value.to_string().as_str())
    }
}

/// generic database result
pub type DbResult<T> = Result<T, DbError>;

/// Syntax sugar type denoting a singular result from the database
pub type DbResultSingle<T> = DbResult<T>;
/// Syntax sugar type denoting multiple results from the database
pub type DbResultMultiple<T> = DbResult<Vec<T>>;
