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
        game_match::GameMatchGetById,
        odds::{Odds, OddsCreate, OddsGetById},
    },
};

pub struct OddsRepository {
    pool_handler: PoolHandler,
}

impl OddsRepository {
    pub async fn get_odds<'a>(
        params: OddsGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Odds>> {
        let bet = sqlx::query_as!(
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

        Ok(bet)
    }

    pub fn is_correct(odds: Option<Odds>) -> DbResultSingle<Odds> {
        match odds {
            Some(odds) if odds.deleted_at.is_none() => Ok(odds),
            Some(_) => Err(BusinessLogicError::new(OddsDeleted).into()),
            None => Err(BusinessLogicError::new(OddsDoNotExist).into()),
        }
    }
}

#[async_trait]
impl DbRepository for OddsRepository {
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
impl DbCreate<OddsCreate, Odds> for OddsRepository {
    #[inline]
    async fn create(&mut self, data: &OddsCreate) -> DbResultSingle<Odds> {
        // TODO: check game_match_id validity against GameMatch

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

        Ok(odds)
    }
}

#[async_trait]
impl DbReadOne<OddsGetById, Odds> for OddsRepository {
    async fn read_one(&mut self, params: &OddsGetById) -> DbResultSingle<Odds> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let odds = Self::is_correct(Self::get_odds(OddsGetById { id: params.id }, &mut tx).await?)?;

        tx.commit().await?;

        Ok(odds)
    }
}

#[async_trait]
impl DbReadMany<GameMatchGetById, Odds> for OddsRepository {
    async fn read_many(&mut self, data: &GameMatchGetById) -> DbResultMultiple<Odds> {
        let mut tx = self.pool_handler.pool.begin().await?;
        // TODO: check GameMatch validity

        let bets = sqlx::query_as!(
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

        Ok(bets)
    }
}

// TODO: odds deletion?
