#![allow(clippy::needless_raw_string_hashes)]

use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

use crate::{
    common::{
        error::{
            BusinessLogicError, BusinessLogicErrorKind::GameDeleted,
            BusinessLogicErrorKind::GameDoesNotExist, DbResultMultiple, DbResultSingle,
        },
        repository::{
            DbCreate, DbDelete, DbPoolHandler, DbReadMany, DbReadOne, DbRepository, DbUpdate,
            PoolHandler,
        },
    },
    models::game::{Game, GameCreate, GameDelete, GameFilter, GameGenre, GameGetById, GameUpdate},
};

pub struct GameRepository {
    pool_handler: PoolHandler,
}

impl GameRepository {
    /// # Panics
    /// # Errors
    pub async fn get_game<'a>(
        params: GameGetById,
        transaction_handle: &mut Transaction<'a, Postgres>,
    ) -> DbResultSingle<Option<Game>> {
        let game = sqlx::query_as!(
            Game,
            r#"
                SELECT id,
                    name,
                    description,
                    logo,
                    genre AS "genre: _",
                    created_at,
                    edited_at,
                    deleted_at
                FROM Game
                WHERE id = $1
            "#,
            params.id
        )
        .fetch_optional(transaction_handle.as_mut())
        .await?;

        Ok(game)
    }

    /// # Errors
    pub fn is_correct(game: Option<Game>) -> DbResultSingle<Game> {
        match game {
            Some(game) if game.deleted_at.is_none() => Ok(game),
            Some(_) => Err(BusinessLogicError::new(GameDeleted).into()),
            None => Err(BusinessLogicError::new(GameDoesNotExist).into()),
        }
    }
}

#[async_trait]
impl DbRepository for GameRepository {
    fn new(pool_handler: PoolHandler) -> Self {
        Self { pool_handler }
    }

    async fn disconnect(&mut self) -> () {
        self.pool_handler.disconnect().await;
    }
}

#[async_trait]
impl DbCreate<GameCreate, Game> for GameRepository {
    async fn create(&mut self, data: &GameCreate) -> DbResultSingle<Game> {
        let game = sqlx::query_as!(
            Game,
            r#"
                INSERT INTO Game (name, description, logo, genre)
                VALUES ($1, $2, $3, $4)
                RETURNING id,
                    name,
                    description,
                    logo,
                    genre AS "genre: _",
                    created_at,
                    edited_at,
                    deleted_at
            "#,
            data.name,
            data.description,
            data.logo,
            data.genre as _,
        )
        .fetch_one(self.pool_handler.pool.as_ref())
        .await?;

        Ok(game)
    }
}

#[async_trait]
impl DbUpdate<GameUpdate, Game> for GameRepository {
    async fn update(&mut self, data: &GameUpdate) -> DbResultMultiple<Game> {
        let mut tx = self.pool_handler.pool.begin().await?;

        GameRepository::is_correct(
            GameRepository::get_game(GameGetById { id: data.id }, &mut tx).await?,
        )?;

        let games = sqlx::query_as!(
            Game,
            r#"
                UPDATE Game
                SET name = COALESCE($1, name),
                    description = COALESCE($2, description),
                    logo = COALESCE($3, logo),
                    genre = COALESCE($4, genre),
                    edited_at = now()
                WHERE id = $5
                RETURNING id,
                    name,
                    description,
                    logo,
                    genre AS "genre: _",
                    created_at,
                    edited_at,
                    deleted_at
            "#,
            data.name,
            data.description,
            data.logo,
            data.genre as _,
            data.id,
        )
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(games)
    }
}

#[async_trait]
impl DbReadOne<GameGetById, Game> for GameRepository {
    async fn read_one(&mut self, params: &GameGetById) -> DbResultSingle<Game> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let game_ok = GameRepository::is_correct(
            GameRepository::get_game(GameGetById { id: params.id }, &mut tx).await?,
        )?;

        tx.commit().await?;

        Ok(game_ok)
    }
}

#[async_trait]
impl DbReadMany<GameFilter, Game> for GameRepository {
    async fn read_many(&mut self, filter: &GameFilter) -> DbResultMultiple<Game> {
        let mut tx = self.pool_handler.pool.begin().await?;

        let mut query_builder = sqlx::QueryBuilder::new(
            r"
                SELECT id,
                    name,
                    description,
                    logo,
                    genre,
                    created_at,
                    edited_at,
                    deleted_at
                FROM Game
                WHERE deleted_at IS NULL
            ",
        );

        if let Some(name) = &filter.name {
            query_builder.push(r" AND name ILIKE concat('%', ");
            query_builder.push_bind(name);
            query_builder.push(r", '%')");
        }

        if let Some(genre) = &filter.genre {
            query_builder.push(r" AND genre = ");
            query_builder.push_bind(genre.clone() as GameGenre);
        }

        let games = query_builder
            .build_query_as::<Game>()
            .fetch_all(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(games)
    }
}

#[async_trait]
impl DbDelete<GameDelete, Game> for GameRepository {
    async fn delete(&mut self, params: &GameDelete) -> DbResultMultiple<Game> {
        let mut tx = self.pool_handler.pool.begin().await?;

        GameRepository::is_correct(
            GameRepository::get_game(GameGetById { id: params.id }, &mut tx).await?,
        )?;

        let games = sqlx::query_as!(
            Game,
            r#"
                UPDATE Game
                SET deleted_at = now()
                WHERE id = $1
                RETURNING id,
                    name,
                    description,
                    logo,
                    genre AS "genre: _",
                    created_at,
                    edited_at,
                    deleted_at
            "#,
            params.id
        )
        .fetch_all(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(games)
    }
}
