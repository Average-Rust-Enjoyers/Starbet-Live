#![allow(dead_code)]
use sqlx::{Transaction, Postgres};
use crate::models::money_transaction::{MoneyTransactionGetById, MoneyTransaction, MoneyTransactionStatus};
use crate::common::error::DbResultSingle;


pub struct MoneyTransactionRepository {
    // pool_handler: PoolHandler,
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
        Ok(
            sqlx::query_as!(MoneyTransaction,
                r#"
                    SELECT 
                        id,
                        app_user_id,
                        status AS "status: MoneyTransactionStatus",
                        amount_tokens,
                        amount_currency,
                        currency AS "currency: _",
                        deposit,
                        created_at,
                        edited_at,
                        deleted_at
                    FROM MoneyTransaction
                    WHERE id = $1
                "#, params.id)
                .fetch_optional(transaction_handle.as_mut())
                .await?,
        )
    }

}