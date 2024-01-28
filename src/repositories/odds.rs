use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

use crate::{
    common::{
        error::{
            BusinessLogicError,
            BusinessLogicErrorKind::{OddsDeleted, OddsDoNotExist},
            DbResultMultiple, DbResultSingle,
        },
        repository::{DbCreate, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, PoolHandler},
    },
    models::{
        bet::BetGetById,
        game_match::GameMatchGetById,
        odds::{Odds, OddsCreate, OddsGetByBetId, OddsGetById},
    },
    DbDelete,
};

use super::{bet::BetRepository, game_match::GameMatchRepository};

#[derive(Clone)]
pub struct OddsRepository {
    pool_handler: PoolHandler,
}

impl OddsRepository {
    pub fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    /// # Panics
    /// # Errors
    pub async fn get_odds<'a>(
        params: OddsGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Odds>> {
        let odds = sqlx::query_as!(
            Odds,
            r#"
                SELECT *
                FROM odds
                WHERE id = $1
            "#,
            params.id
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;

        Ok(odds)
    }

    /// # Errors
    pub fn is_correct(odds: Option<Odds>) -> DbResultSingle<Odds> {
        match odds {
            Some(odds) if odds.deleted_at.is_none() => Ok(odds),
            Some(_) => Err(BusinessLogicError::new(OddsDeleted).into()),
            None => Err(BusinessLogicError::new(OddsDoNotExist).into()),
        }
    }

    pub async fn get_closest_odds_for_bet<'a>(
        params: OddsGetByBetId,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Odds>> {
        let bet = BetRepository::is_correct(
            BetRepository::get_bet(BetGetById { id: params.bet_id }, transaction_handle).await?,
        )?;

        let odds = sqlx::query_as!(
            Odds,
            r#"
                SELECT *
                FROM odds
                WHERE game_match_id = $1
                ORDER BY CASE
                    WHEN $2 > created_at
                        THEN $2 - created_at
                        ELSE created_at - $2
                    END
                    ASC
                LIMIT 1
            "#,
            bet.game_match_id,
            bet.created_at
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;

        Ok(odds)
    }
}

#[async_trait]
impl DbRepository for OddsRepository {
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    async fn disconnect(&mut self) -> () {
        self.pool_handler.disconnect().await;
    }
}

#[async_trait]
impl DbCreate<OddsCreate, Odds> for OddsRepository {
    async fn create(&mut self, data: &OddsCreate) -> DbResultSingle<Odds> {
        let mut tx = self.pool_handler.pool.begin().await?;

        GameMatchRepository::is_correct(
            GameMatchRepository::get_game_match(
                GameMatchGetById {
                    id: data.game_match_id,
                },
                &mut tx,
            )
            .await?,
        )?;

        let odds = sqlx::query_as!(
            Odds,
            r#"
                INSERT INTO Odds
                VALUES ($1, $2, $3, $4)
                RETURNING *
            "#,
            data.id,
            data.game_match_id,
            data.odds_a,
            data.odds_b
        )
        .fetch_one(self.pool_handler.pool.as_ref())
        .await?;

        tx.commit().await?;

        Ok(odds)
    }
}

#[async_trait]
impl DbReadOne<OddsGetById, Odds> for OddsRepository {
    async fn read_one(&mut self, params: &OddsGetById) -> DbResultSingle<Odds> {
        let mut tx = self.pool_handler.pool.begin().await?;

        Ok(Self::is_correct(
            Self::get_odds(params.clone(), &mut tx).await?,
        )?)
    }
}

#[async_trait]
impl DbReadMany<GameMatchGetById, Odds> for OddsRepository {
    async fn read_many(&mut self, data: &GameMatchGetById) -> DbResultMultiple<Odds> {
        let mut tx = self.pool_handler.pool.begin().await?;

        GameMatchRepository::is_correct(
            GameMatchRepository::get_game_match(data.clone(), &mut tx).await?,
        )?;

        let odds = sqlx::query_as!(
            Odds,
            r#"
                SELECT *
                FROM Odds
                WHERE deleted_at IS NULL
                AND game_match_id = $1
            "#,
            data.id,
        )
        .fetch_all(&mut *tx)
        .await?;

        Ok(odds)
    }
}

#[async_trait]
impl DbDelete<OddsGetById, Odds> for OddsRepository {
    async fn delete(&mut self, params: &OddsGetById) -> DbResultMultiple<Odds> {
        let mut tx = self.pool_handler.pool.begin().await?;

        Self::is_correct(Self::get_odds(params.clone(), &mut tx).await?)?;

        let odds = sqlx::query_as!(
            Odds,
            r#"
                UPDATE Odds
                SET deleted_at = now()
                WHERE id = $1
                RETURNING *
            "#,
            params.id,
        )
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(odds)
    }
}
