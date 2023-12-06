#![allow(dead_code)]

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Currency {
    CZK,
    EUR,
    USD,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MoneyTransactionStatus {
    Pending,
    Completed,
    Cancelled,
}

#[derive(sqlx::FromRow, Debug, PartialEq, Clone)]
pub struct MoneyTransaction {
    pub id: Uuid,
    // ----------
    pub app_user_id: Uuid,
    // ----------
    pub status: MoneyTransactionStatus,
    pub amout_tokens: i32,
    pub amount_currency: f32,
    pub currency: Currency,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MoneyTransactionCreate {
    pub name: String,
    pub app_user_id: Uuid,
    pub amount_tokens: i32,
    pub amount_currency: f32,
    pub currency: Currency,
}

impl MoneyTransactionCreate {
    pub fn new(
        name: &str,
        app_user_id: &Uuid,
        amount_tokens: i32,
        amount_currency: f32,
        currency: Currency,
    ) -> Self {
        Self {
            name: name.to_owned(),
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
    pub fn new(id: &Uuid, status: MoneyTransactionStatus) -> Self {
        let change_to_owned = |value: &str| Some(value.to_owned());
        Self { id: *id, status }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MoneyTransactionDelete {
    pub id: Uuid,
}

impl MoneyTransactionDelete {
    pub fn new(id: &Uuid) -> Self {
        Self { id: *id }
    }
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
