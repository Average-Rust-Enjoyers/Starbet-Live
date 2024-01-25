#[cfg(test)]
pub mod money_transaction_repo_tests {
    use chrono::DateTime;
    use sqlx::PgPool;
    use starbet_live::{
        common::{
            repository::{DbRepository, PoolHandler},
            DbUpdateOne,
        },
        error::DbResultSingle,
        models::{
            money_transaction::{
                Currency, MoneyTransaction, MoneyTransactionGetById, MoneyTransactionStatus,
                MoneyTransactionUpdateStatus,
            },
            user::GetByUserId,
        },
        repositories::money_transaction::MoneyTransactionRepository,
        DbPoolHandler, DbReadMany, DbReadOne,
    };
    use std::sync::Arc;
    use uuid::Uuid;

    #[sqlx::test(fixtures("appuser", "money_transaction"))]
    async fn read_one(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let mut money_transaction_repository =
            MoneyTransactionRepository::new(PoolHandler::new(arc_pool));

        let app_user_id = Uuid::parse_str("f2464e56-6719-44ef-b490-df6738a9c11a").unwrap();

        let mut pending_money_transaction = MoneyTransaction {
            id: Uuid::parse_str("6444352d-6c6a-4625-aab5-7df808575901").unwrap(),
            app_user_id,
            status: MoneyTransactionStatus::Pending,
            amount_tokens: 14445,
            amount_currency: 183.59,
            currency: Currency::EUR,
            deposit: false,
            created_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00")
                .unwrap()
                .into(),
            edited_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00")
                .unwrap()
                .into(),
        };

        let money_transaction_by_id = money_transaction_repository
            .read_one(&MoneyTransactionGetById {
                id: pending_money_transaction.id,
            })
            .await
            .expect("The repository call should succeed - id parameter given");

        pending_money_transaction.created_at = money_transaction_by_id.created_at;
        pending_money_transaction.edited_at = money_transaction_by_id.edited_at;

        assert_eq!(pending_money_transaction, money_transaction_by_id);
        money_transaction_repository.disconnect().await;
        Ok(())
    }

    #[sqlx::test(fixtures("appuser", "money_transaction"))]
    async fn update_status(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let mut money_transaction_repository =
            MoneyTransactionRepository::new(PoolHandler::new(arc_pool));

        let app_user_id = Uuid::parse_str("f2464e56-6719-44ef-b490-df6738a9c11a").unwrap();

        let pending_money_transaction = MoneyTransaction {
            id: Uuid::parse_str("6444352d-6c6a-4625-aab5-7df808575901").unwrap(),
            app_user_id,
            status: MoneyTransactionStatus::Pending,
            amount_tokens: 14445,
            amount_currency: 183.59,
            currency: Currency::EUR,
            deposit: false,
            created_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00")
                .unwrap()
                .into(),
            edited_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00")
                .unwrap()
                .into(),
        };

        let changed_status_transaction = money_transaction_repository
            .update(&MoneyTransactionUpdateStatus {
                id: pending_money_transaction.id,
                status: MoneyTransactionStatus::Completed,
            })
            .await
            .expect("The repository call should succeed - id and status parameters given");

        let mut completed_money_transaction = MoneyTransaction {
            status: MoneyTransactionStatus::Completed,
            ..pending_money_transaction.clone()
        };

        completed_money_transaction.created_at = changed_status_transaction.created_at;
        completed_money_transaction.edited_at = changed_status_transaction.edited_at;

        assert_eq!(completed_money_transaction, changed_status_transaction);

        money_transaction_repository.disconnect().await;
        Ok(())
    }

    #[sqlx::test(fixtures("appuser", "money_transaction"))]
    async fn read_many(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let mut money_transaction_repository =
            MoneyTransactionRepository::new(PoolHandler::new(arc_pool));

        let app_user_id = Uuid::parse_str("f2464e56-6719-44ef-b490-df6738a9c11a").unwrap();

        let completed_money_transaction = MoneyTransaction {
            id: Uuid::parse_str("6159723d-107e-490c-9c3c-bbfdc2c03f3c").unwrap(),
            app_user_id,
            status: MoneyTransactionStatus::Completed,
            amount_tokens: 20971,
            amount_currency: 142.06,
            currency: Currency::USD,
            deposit: false,
            created_at: DateTime::parse_from_rfc3339("2024-01-10 17:51:47.988874+00:00")
                .unwrap()
                .into(),
            edited_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00")
                .unwrap()
                .into(),
        };

        let pending_money_transaction = MoneyTransaction {
            id: Uuid::parse_str("6444352d-6c6a-4625-aab5-7df808575901").unwrap(),
            app_user_id,
            status: MoneyTransactionStatus::Pending,
            amount_tokens: 14445,
            amount_currency: 183.59,
            currency: Currency::EUR,
            deposit: false,
            created_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00")
                .unwrap()
                .into(),
            edited_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00")
                .unwrap()
                .into(),
        };

        let mut money_transactions_by_user_id = money_transaction_repository
            .read_many(&GetByUserId { id: app_user_id })
            .await
            .expect("The repository call should succeed - app_user_id parameter given");

        money_transactions_by_user_id[0].created_at = pending_money_transaction.created_at;
        money_transactions_by_user_id[0].edited_at = pending_money_transaction.edited_at;
        money_transactions_by_user_id[1].created_at = completed_money_transaction.created_at;
        money_transactions_by_user_id[1].edited_at = completed_money_transaction.edited_at;

        assert_eq!(
            money_transactions_by_user_id,
            vec![pending_money_transaction, completed_money_transaction]
        );

        money_transaction_repository.disconnect().await;
        Ok(())
    }
}
