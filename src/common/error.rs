use std::fmt::{Debug, Display, Formatter};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

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
    BetAmountNotAllowed,
    InsufficientFunds,
    // MoneyTransaction errors
    MoneyTransactionDoesNotExist,
    MoneyTransactionAmountTooLow,
    MoneyTransactionAmountNotAllowed,
    // Odds errors
    OddsDoNotExist,
    OddsDeleted,

    // --------------------------
    // Generic errors
    UserUpdateParametersEmpty,
}

impl Display for BusinessLogicErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let does_not_exist = |name: &str| format!("The specified {name} does not exist!");
        let deleted = |name: &str| format!("The specified {name} has been deleted!");
        let amount_too_low = |name: &str| format!("The specified {name} ammount is too low!");
        let amount_not_allowed =
            |name: &str| format!("The specified {name} ammount is not allowed!");

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
            BusinessLogicErrorKind::BetAmountTooLow => amount_too_low("bet"),
            BusinessLogicErrorKind::BetAmountNotAllowed => amount_not_allowed("bet"),
            BusinessLogicErrorKind::InsufficientFunds => {
                "The user has insufficient funds to place the bet!".to_string()
            }
            BusinessLogicErrorKind::MoneyTransactionDoesNotExist => {
                does_not_exist("money transaction")
            }
            BusinessLogicErrorKind::MoneyTransactionAmountTooLow => {
                amount_too_low("money transaction")
            }
            BusinessLogicErrorKind::MoneyTransactionAmountNotAllowed => {
                amount_not_allowed("money transaction")
            }
            BusinessLogicErrorKind::OddsDoNotExist => does_not_exist("odds"),
            BusinessLogicErrorKind::OddsDeleted => deleted("odds"),
        };
        f.write_str(error_string.as_str())
    }
}

/// Error type representing a Business Logic Error in the database layer ->
/// usually a problem with missing records, insufficient rights for operation, and so on.
pub struct BusinessLogicError {
    pub error: BusinessLogicErrorKind,
}

impl BusinessLogicError {
    /// Business Logic Error constructor
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

#[derive(Debug)]
pub enum ExternalApiError {
    DbError(DbError),
    Error(String),
    GraphQl(Vec<cynic::GraphQlError>),
}

impl From<&str> for ExternalApiError {
    fn from(err: &str) -> Self {
        Self::Error(err.to_owned())
    }
}

impl From<DbError> for ExternalApiError {
    fn from(err: DbError) -> Self {
        Self::DbError(err)
    }
}

impl From<Vec<cynic::GraphQlError>> for ExternalApiError {
    fn from(err: Vec<cynic::GraphQlError>) -> Self {
        Self::GraphQl(err)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Status code: {0:?}")]
    StatusCode(StatusCode),
    #[error("Logic error: {0:?}")]
    BusinessLogicError(BusinessLogicErrorKind),
    #[error("Parsing error")]
    UuidError(#[from] uuid::Error),
    #[error("Parsing error")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Parsing error")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("Auth error")]
    AuthenticationError(#[from] axum_login::Error<crate::auth::Auth>),
    #[error("Websocket fail")]
    WebSocketError(barrage::SendError<std::string::String>),
    #[error("Forbidden error")]
    ForbiddenError,
    #[error("Database error")]
    DbError(#[from] DbError),
    #[error("Invalid request")]
    TemplatingError(#[from] askama::Error),
}

impl From<barrage::SendError<std::string::String>> for AppError {
    fn from(err: barrage::SendError<std::string::String>) -> Self {
        Self::WebSocketError(err)
    }
}

impl From<StatusCode> for AppError {
    fn from(err: StatusCode) -> Self {
        Self::StatusCode(err)
    }
}

pub type AppResult<T> = Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match self {
            AppError::StatusCode(status_code) => status_code,
            AppError::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
            AppError::ForbiddenError => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, self.to_string()).into_response()
    }
}
