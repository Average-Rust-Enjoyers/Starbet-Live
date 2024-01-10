use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

use crate::{
    common::{
        error::{
            BusinessLogicError, BusinessLogicErrorKind::BetDeleted,
            BusinessLogicErrorKind::BetDoesNotExist, DbResultMultiple, DbResultSingle,
        },
        repository::{
            DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, DbUpdate,
            PoolHandler,
        },
    },
    models::bet::{Bet, BetCreate, BetDelete, BetGetById, BetGetByUserId, BetUpdate},
};

pub struct BetRepository {
    pool_handler: PoolHandler,
}

impl BetRepository {
    pub async fn get_bet<'a>(
        params: BetGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Bet>> {
        let bet = sqlx::query_as!(
            Bet,
            r#"
                SELECT
                    id,
                    app_user_id,
                    game_match_id,
                    amount,
                    status AS "status: _",
                    expected_outcome AS "expected_outcome: _",
                    created_at,
                    edited_at,
                    deleted_at
                FROM Bet
                WHERE id = $1
            "#,
            params.id
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;

        Ok(bet)
    }

    pub fn is_correct(bet: Option<Bet>) -> DbResultSingle<Bet> {
        match bet {
            Some(bet) if bet.deleted_at.is_none() => Ok(bet),
            Some(_) => Err(BusinessLogicError::new(BetDeleted).into()),
            None => Err(BusinessLogicError::new(BetDoesNotExist).into()),
        }
    }
}

#[async_trait]
impl DbRepository for BetRepository {
    #[inline]
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    #[inline]
    async fn disconnect(&mut self) -> () {
        self.pool_handler.disconnect().await;
    }
}

#[async_trait]
impl DbCreate<BetCreate, Bet> for BetRepository {
    #[inline]
    async fn create(&mut self, data: &BetCreate) -> DbResultSingle<Bet> {
        let bet = sqlx::query_as!(
            Bet,
            r#"
                INSERT INTO Bet (id, app_user_id, game_match_id, amount, expected_outcome)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING
                    id,
                    app_user_id,
                    game_match_id,
                    amount,
                    status AS "status: _",
                    expected_outcome AS "expected_outcome: _",
                    created_at,
                    edited_at,
                    deleted_at
            "#,
            data.id,
            data.app_user_id,
            data.game_match_id,
            data.amount,
            data.expected_outcome as _,
        )
        .fetch_one(self.pool_handler.pool.as_ref())
        .await?;
        // TODO: check if the user has enough money to place the bet, if not error InsufficientFunds
        Ok(bet)
    }
}

#[async_trait]
impl DbUpdate<BetUpdate, Bet> for BetRepository {
    async fn update(&mut self, data: &BetUpdate) -> DbResultMultiple<Bet> {
        let mut tx = self.pool_handler.pool.begin().await?;

        BetRepository::is_correct(
            BetRepository::get_bet(BetGetById { id: data.id }, &mut tx).await?,
        )?;

        let bets = sqlx::query_as!(
            Bet,
            r#"
                UPDATE Bet
                SET status = COALESCE($2, status),
                    edited_at = now()
                WHERE id = $1
                RETURNING
                    id,
                    app_user_id,
                    game_match_id,
                    amount,
                    status AS "status: _",
                    expected_outcome AS "expected_outcome: _",
                    created_at,
                    edited_at,
                    deleted_at
            "#,
            data.id,
            data.status as _,
        )
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(bets)
    }
}

#[async_trait]
impl DbReadOne<BetGetById, Bet> for BetRepository {
    async fn read_one(&mut self, params: &BetGetById) -> DbResultSingle<Bet> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let bet_ok = BetRepository::is_correct(
            BetRepository::get_bet(BetGetById { id: params.id }, &mut tx).await?,
        )?;

        tx.commit().await?;

        Ok(bet_ok)
    }
}

#[async_trait]
impl DbReadMany<BetGetByUserId, Bet> for BetRepository {
    async fn read_many(&mut self, data: &BetGetByUserId) -> DbResultMultiple<Bet> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let bets = sqlx::query_as!(
            Bet,
            r#"
                SELECT
                    id,
                    app_user_id,
                    game_match_id,
                    amount,
                    status AS "status: _",
                    expected_outcome AS "expected_outcome: _",
                    created_at,
                    edited_at,
                    deleted_at
                FROM Bet
                WHERE deleted_at IS NULL
                AND app_user_id = $1
            "#,
            data.user_id,
        )
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(bets)
    }
}

#[async_trait]
impl DbDelete<BetDelete, Bet> for BetRepository {
    async fn delete(&mut self, params: &BetDelete) -> DbResultMultiple<Bet> {
        let mut tx = self.pool_handler.pool.begin().await?;

        BetRepository::is_correct(
            BetRepository::get_bet(BetGetById { id: params.id }, &mut tx).await?,
        )?;

        let bets = sqlx::query_as!(
            Bet,
            r#"
                UPDATE Bet
                SET deleted_at = now()
                WHERE id = $1
                RETURNING
                    id,
                    app_user_id,
                    game_match_id,
                    amount,
                    status AS "status: _",
                    expected_outcome AS "expected_outcome: _",
                    created_at,
                    edited_at,
                    deleted_at
            "#,
            params.id
        )
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(bets)
    }
}
