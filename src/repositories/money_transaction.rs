#![allow(dead_code)]
use crate::common::error::BusinessLogicErrorKind;
use crate::common::error::{BusinessLogicError, DbError, DbResultSingle};
use crate::common::repository::{DbCreate, DbReadOne, PoolHandler};
use crate::repositories::money_transaction::BusinessLogicErrorKind::MoneyTransactionDoesNotExist;
use crate::models::money_transaction::{
    MoneyTransaction, MoneyTransactionCreate, MoneyTransactionGetById,
};
use async_trait::async_trait;
use sqlx::{Acquire, Postgres, Transaction};

pub struct MoneyTransactionRepository {
    pool_handler: PoolHandler,
}

impl MoneyTransactionRepository {
    /// Function which retrieves a single money transaction by its id, usable within a database transaction
    ///
    /// # Params
    /// - `params`: structure containing the ID of the money transaction
    /// - `transaction_handle` mutable reference to an ongoing database transaction
    ///
    /// # Returns
    /// - `Ok(transaction)`: on successful connection and retrieval
    /// - `Err(_)`: otherwise
    pub(crate) async fn get_money_transaction<'a>(
        params: MoneyTransactionGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<MoneyTransaction>> {
        Ok(sqlx::query_as!(
            MoneyTransaction,
            r#"
                    SELECT 
                        id,
                        app_user_id,
                        status AS "status: _",
                        amount_tokens,
                        amount_currency,
                        currency AS "currency: _",
                        deposit,
                        created_at,
                        edited_at
                    FROM MoneyTransaction
                    WHERE id = $1
                "#,
            params.id
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?)
    }

    /// Function which checks if the money transaction is correct (existing and not deleted)
    ///
    /// # Params
    /// - `transaction`: optional money transaction retrieved from the database
    ///
    /// # Returns
    /// - `Ok(post)`: when the transaction exists and is not deleted
    /// - `Err(DbError)`: with appropriate error description otherwise
    pub(crate) fn is_correct(tx: Option<MoneyTransaction>) -> DbResultSingle<MoneyTransaction> {
        match tx {
            Some(tx) => Ok(tx),
            None => Err(BusinessLogicError::new(MoneyTransactionDoesNotExist).into()),
        }
    }
}

#[async_trait]
impl DbCreate<MoneyTransactionCreate, MoneyTransaction> for MoneyTransactionRepository {
    async fn create(&mut self, data: &MoneyTransactionCreate) -> DbResultSingle<MoneyTransaction> {
        let mut tx = self.pool_handler.pool.begin().await?;
        // TODO: check if User exists and is not deleted

        let conn = tx.acquire().await?;
        Ok(sqlx::query_as!(
            MoneyTransaction,
            r#"
                INSERT INTO MoneyTransaction
                (
                    app_user_id,
                    amount_tokens,
                    amount_currency,
                    currency
                )
                VALUES ($1, $2, $3, $4)
                RETURNING 
                    id,
                    app_user_id,
                    status AS "status: _",
                    amount_tokens,
                    amount_currency,
                    currency AS "currency: _",
                    deposit,
                    created_at,
                    edited_at
            "#,
            data.app_user_id,
            data.amount_tokens,
            data.amount_currency,
            data.currency as _
        )
        .fetch_one(conn)
        .await?)
    }
}

#[async_trait]
impl DbReadOne<MoneyTransactionGetById, MoneyTransaction> for MoneyTransactionRepository {
    async fn read_one(
        &mut self,
        params: &MoneyTransactionGetById,
    ) -> DbResultSingle<MoneyTransaction> {
        let mut tx = self.pool_handler.pool.begin().await?;

        // TODO: check if user (transaction owner) exists and is not deleted
        MoneyTransactionRepository::is_correct(
            MoneyTransactionRepository::get_money_transaction(params.clone(), &mut tx).await?,
        )
    }
}
