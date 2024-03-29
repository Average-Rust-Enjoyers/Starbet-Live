use chrono::{DateTime, Utc};
use uuid::Uuid;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Clone, sqlx::Type)]
pub enum Currency {
    CZK,
    EUR,
    USD,
}

#[derive(Debug, PartialEq, Eq, Clone, sqlx::Type)]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MoneyTransactionStatus {
    Pending,
    Completed,
    Canceled,
}

#[derive(sqlx::FromRow, Debug, PartialEq, Clone)]
pub struct MoneyTransaction {
    pub id: Uuid,
    // ----------
    pub app_user_id: Uuid,
    // ----------
    pub status: MoneyTransactionStatus,
    pub amount_tokens: i32,
    pub amount_currency: f64,
    pub currency: Currency,
    pub deposit: bool,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MoneyTransactionCreate {
    pub app_user_id: Uuid,
    pub amount_tokens: i32,
    pub amount_currency: f64,
    pub currency: Currency,
}

impl MoneyTransactionCreate {
    #[allow(dead_code)]
    pub fn new(
        app_user_id: &Uuid,
        amount_tokens: i32,
        amount_currency: f64,
        currency: Currency,
    ) -> Self {
        Self {
            app_user_id: app_user_id.to_owned(),
            amount_tokens,
            amount_currency,
            currency,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MoneyTransactionUpdateStatus {
    pub id: Uuid,
    pub status: MoneyTransactionStatus,
}

impl MoneyTransactionUpdateStatus {
    #[allow(dead_code)]
    pub fn new(id: &Uuid, status: MoneyTransactionStatus) -> Self {
        Self { id: *id, status }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MoneyTransactionDelete {
    pub id: Uuid,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MoneyTransactionGetById {
    pub id: Uuid,
}

impl MoneyTransactionGetById {
    pub fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
}

impl From<&MoneyTransactionDelete> for MoneyTransactionGetById {
    fn from(money_transaction_id: &MoneyTransactionDelete) -> Self {
        Self {
            id: money_transaction_id.id,
        }
    }
}
