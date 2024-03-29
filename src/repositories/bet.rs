use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

use crate::{
    common::{
        error::{
            BusinessLogicError, BusinessLogicErrorKind::BetDeleted,
            BusinessLogicErrorKind::BetDoesNotExist, DbResultMultiple, DbResultSingle,
        },
        repository::{DbCreate, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, PoolHandler},
        DbUpdateOne,
    },
    models::{
        bet::{Bet, BetCreate, BetDelete, BetGetById, BetGetByMatchId, BetGetByUserId, BetUpdate},
        user::UserUpdateBalance,
    },
    DbDelete,
};

use super::user::UserRepository;

#[derive(Clone)]
pub struct BetRepository {
    pool_handler: PoolHandler,
}

impl BetRepository {
    /// # Panics
    /// # Errors
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
                    odds_id,
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

    pub async fn get_bets_for_game<'a>(
        params: BetGetByMatchId,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultMultiple<Bet> {
        let bets: Vec<Bet> = sqlx::query_as!(
            Bet,
            r#"
                SELECT
                    id,
                    app_user_id,
                    game_match_id,
                    odds_id,
                    amount,
                    status AS "status: _",
                    expected_outcome AS "expected_outcome: _",
                    created_at,
                    edited_at,
                    deleted_at
                FROM Bet
                WHERE game_match_id = $1
            "#,
            params.match_id
        )
        .fetch_all(transaction_handle.as_mut())
        .await?;

        Ok(bets
            .into_iter()
            .flat_map(|b| Self::is_correct(Some(b)))
            .collect())
    }

    pub async fn update_bet<'a>(
        params: BetUpdate,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Bet> {
        let bet = sqlx::query_as!(
            Bet,
            r#"
                UPDATE Bet
                SET status = $2,
                    edited_at = now()
                WHERE id = $1
                RETURNING
                    id,
                    app_user_id,
                    game_match_id,
                    odds_id,
                    amount,
                    status AS "status: _",
                    expected_outcome AS "expected_outcome: _",
                    created_at,
                    edited_at,
                    deleted_at
            "#,
            params.id,
            params.status as _,
        )
        .fetch_one(transaction_handle.as_mut())
        .await?;

        Ok(bet)
    }

    /// # Errors
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
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    async fn disconnect(&mut self) -> () {
        self.pool_handler.disconnect().await;
    }
}

#[async_trait]
impl DbCreate<BetCreate, Bet> for BetRepository {
    async fn create(&mut self, data: &BetCreate) -> DbResultSingle<Bet> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let bet = sqlx::query_as!(
            Bet,
            r#"
                INSERT INTO Bet (id, app_user_id, game_match_id, odds_id, amount, expected_outcome)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING
                    id,
                    app_user_id,
                    game_match_id,
                    odds_id,
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
            data.odds_id,
            data.amount,
            data.expected_outcome as _,
        )
        .fetch_one(self.pool_handler.pool.as_ref())
        .await?;

        UserRepository::update_user_balance(
            UserUpdateBalance {
                id: data.app_user_id,
                delta: -bet.amount,
            },
            &mut tx,
        )
        .await?;

        tx.commit().await?;
        Ok(bet)
    }
}

#[async_trait]
impl DbUpdateOne<BetUpdate, Bet> for BetRepository {
    async fn update(&mut self, data: &BetUpdate) -> DbResultSingle<Bet> {
        let mut tx = self.pool_handler.pool.begin().await?;

        Self::is_correct(Self::get_bet(BetGetById { id: data.id }, &mut tx).await?)?;
        let bet = Self::update_bet(data.clone(), &mut tx).await?;

        tx.commit().await?;
        Ok(bet)
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
                    odds_id,
                    amount,
                    status AS "status: _",
                    expected_outcome AS "expected_outcome: _",
                    created_at,
                    edited_at,
                    deleted_at
                FROM Bet
                WHERE deleted_at IS NULL
                AND app_user_id = $1
                ORDER BY created_at DESC
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
                    odds_id,
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
