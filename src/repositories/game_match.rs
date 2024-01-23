use async_trait::async_trait;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::{
    common::{
        error::{
            BusinessLogicError,
            BusinessLogicErrorKind::{GameMatchDeleted, GameMatchDoesNotExist},
            DbResultMultiple, DbResultSingle,
        },
        DbCreate, DbDelete, DbGetByForeignKey, DbPoolHandler, DbReadAll, DbReadOne, DbRepository,
        DbUpdate, PoolHandler,
    },
    models::game_match::{
        GameMatch, GameMatchCreate, GameMatchDelete, GameMatchGetById, GameMatchStatus,
        GameMatchUpdate,
    },
};

#[derive(Clone)]
pub struct GameMatchRepository {
    pool_handler: PoolHandler,
}

impl GameMatchRepository {
    pub fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    /// # Panics
    /// # Errors
    pub async fn get_game_match<'a>(
        params: GameMatchGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<GameMatch>> {
        let game_match = sqlx::query_as!(
            GameMatch,
            r#"SELECT 
                id, 
                game_id, 
                name_a, 
                name_b, 
                starts_at, 
                ends_at, 
                outcome AS "outcome: _", 
                status AS "status: _", 
                created_at, 
                edited_at, 
                deleted_at
             FROM GameMatch gm WHERE gm.id = $1"#,
            params.id
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;

        Ok(game_match)
    }

    /// # Errors
    pub fn is_correct(game_match: Option<GameMatch>) -> DbResultSingle<GameMatch> {
        match game_match {
            Some(game_match) => match game_match.deleted_at {
                Some(_) => Err(BusinessLogicError::new(GameMatchDeleted).into()),
                None => Ok(game_match),
            },
            None => Err(BusinessLogicError::new(GameMatchDoesNotExist).into()),
        }
    }
}

#[async_trait]
impl DbRepository for GameMatchRepository {
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    async fn disconnect(&mut self) -> () {
        self.pool_handler.disconnect().await;
    }
}

#[async_trait]
impl DbCreate<GameMatchCreate, GameMatch> for GameMatchRepository {
    async fn create(&mut self, data: &GameMatchCreate) -> DbResultSingle<GameMatch> {
        let game_match = sqlx::query_as!(
            GameMatch,
            r#"INSERT INTO GameMatch 
            (game_id, name_a, name_b, starts_at, ends_at) 
            VALUES 
            ($1, $2, $3, $4, $5) 
            RETURNING 
                 id, 
                 game_id, 
                 name_a, 
                 name_b, 
                 starts_at, 
                 ends_at, 
                 outcome AS "outcome: _", 
                 status AS "status: _", 
                 created_at, 
                 edited_at, 
                 deleted_at
            "#,
            data.game_id,
            data.name_a,
            data.name_b,
            data.starts_at,
            data.ends_at
        )
        .fetch_one(self.pool_handler.pool.as_ref())
        .await?;

        Ok(game_match)
    }
}

#[async_trait]
impl DbUpdate<GameMatchUpdate, GameMatch> for GameMatchRepository {
    async fn update(&mut self, data: &GameMatchUpdate) -> DbResultMultiple<GameMatch> {
        let mut tx = self.pool_handler.pool.begin().await?;

        GameMatchRepository::is_correct(
            GameMatchRepository::get_game_match(GameMatchGetById { id: data.id }, &mut tx).await?,
        )?;

        let matches = sqlx::query_as!(
            GameMatch,
            r#"UPDATE GameMatch gm SET 
                name_a = COALESCE($1, name_a),
                name_b = COALESCE($2, name_b),
                starts_at = COALESCE($3, starts_at),
                ends_at = COALESCE($4, ends_at),
                status = COALESCE($5, status),
                edited_at = now()
            WHERE gm.id = $6
            RETURNING
                id, 
                game_id, 
                name_a, 
                name_b, 
                starts_at, 
                ends_at, 
                outcome AS "outcome: _", 
                status AS "status: _", 
                created_at, 
                edited_at, 
                deleted_at
            "#,
            data.name_a,
            data.name_b,
            data.starts_at,
            data.ends_at,
            data.status as _,
            data.id
        )
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(matches)
    }
}

#[async_trait]
impl DbReadOne<GameMatchGetById, GameMatch> for GameMatchRepository {
    async fn read_one(&mut self, params: &GameMatchGetById) -> DbResultSingle<GameMatch> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let game_match =
            GameMatchRepository::get_game_match(GameMatchGetById { id: params.id }, &mut tx)
                .await?;

        let game_match_ok = GameMatchRepository::is_correct(game_match)?;

        tx.commit().await?;

        Ok(game_match_ok)
    }
}

#[async_trait]
impl DbDelete<GameMatchDelete, GameMatch> for GameMatchRepository {
    async fn delete(&mut self, params: &GameMatchDelete) -> DbResultMultiple<GameMatch> {
        let mut tx = self.pool_handler.pool.begin().await?;

        GameMatchRepository::is_correct(
            GameMatchRepository::get_game_match(GameMatchGetById { id: params.id }, &mut tx)
                .await?,
        )?;

        let matches = sqlx::query_as!(
            GameMatch,
            r#"UPDATE GameMatch gm SET 
                deleted_at = now()
            WHERE gm.id = $1
            RETURNING
                id, 
                game_id, 
                name_a, 
                name_b, 
                starts_at, 
                ends_at, 
                outcome AS "outcome: _", 
                status AS "status: _", 
                created_at, 
                edited_at, 
                deleted_at
            "#,
            params.id
        )
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(matches)
    }
}

#[async_trait]
impl DbReadAll<GameMatch> for GameMatchRepository {
    async fn read_all(&mut self) -> DbResultMultiple<GameMatch> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let matches = sqlx::query_as!(
            GameMatch,
            r#"SELECT 
                id, 
                game_id, 
                name_a, 
                name_b, 
                starts_at, 
                ends_at, 
                outcome AS "outcome: _", 
                status AS "status: _", 
                created_at, 
                edited_at, 
                deleted_at
             FROM GameMatch gm"#
        )
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(matches)
    }
}

#[async_trait]
impl DbGetByForeignKey<Uuid, GameMatch> for GameMatchRepository {
    async fn get_by_foreign_key(&mut self, game_id: &Uuid) -> DbResultMultiple<GameMatch> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let matches = sqlx::query_as!(
            GameMatch,
            r#"SELECT 
                id, 
                game_id, 
                name_a, 
                name_b, 
                starts_at, 
                ends_at, 
                outcome AS "outcome: _", 
                status AS "status: _", 
                created_at, 
                edited_at, 
                deleted_at
             FROM GameMatch gm WHERE gm.game_id = $1 AND gm.status = $2"#,
            game_id,
            GameMatchStatus::Live as _
        )
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(matches)
    }
}
