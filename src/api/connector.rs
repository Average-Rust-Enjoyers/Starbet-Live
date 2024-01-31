use crate::{
    common::error::ExternalApiError,
    common::DbReadAll,
    models::{
        game::Game,
        game_match::{GameMatch, GameMatchCreate},
    },
    repositories::game_match::GameMatchRepository,
    DbCreate, DbRepository, GameRepository, PoolHandler,
};

#[async_trait::async_trait]
pub trait ExternalApiIntegration<T> {
    async fn fetch_game_matches(self, game: &Game) -> Result<Vec<T>, ExternalApiError>;
    fn into(game_match: T, game: &Game) -> Result<GameMatchCreate, ExternalApiError>;
}

// TODO: better name?
#[derive(Clone)]
pub struct ApiConnector {
    pub game_match_repo: GameMatchRepository,
    pub game_repo: GameRepository,
}

impl ApiConnector {
    pub fn new(pool_handler: PoolHandler) -> Self {
        Self {
            game_match_repo: GameMatchRepository::new(pool_handler.clone()),
            game_repo: GameRepository::new(pool_handler.clone()),
        }
    }

    pub async fn import_external_data<T, F>(self, api: F) -> Result<(), ExternalApiError>
    where
        T: Clone + std::fmt::Debug,
        F: ExternalApiIntegration<T> + Clone + std::fmt::Debug,
    {
        let games = self.clone().game_repo.read_all().await?;
        // TODO - use actual async for loop
        for game in games {
            let game_matches: Vec<T> = api.clone().fetch_game_matches(&game).await?;

            // TODO: collecting to handle erroros after the stage. might be better to use try_for_each or something else
            let game_matches_extracted_data = game_matches
                .into_iter()
                .map(|game_match| F::into(game_match, &game))
                .collect::<Result<Vec<GameMatchCreate>, ExternalApiError>>()?;

            // TODO: fix ugly data manipulation
            let mut stored_game_matches = vec![];
            for game_match_data in game_matches_extracted_data {
                let stored_game_match = self.clone().store_game_match(game_match_data).await;
                if let Some(stored_game_match) = stored_game_match.transpose() {
                    stored_game_matches.push(stored_game_match)
                }
            }

            stored_game_matches
                .into_iter()
                .collect::<Result<Vec<GameMatch>, ExternalApiError>>()?;
        }

        Ok(())
    }

    async fn store_game_match(
        &mut self,
        game_match_data: GameMatchCreate,
    ) -> Result<Option<GameMatch>, ExternalApiError> {
        self.game_match_repo
            .create(&game_match_data)
            .await
            .map_err(ExternalApiError::DbError)
    }

    pub async fn disconnect(&mut self) {
        self.game_match_repo.disconnect().await;
        self.game_repo.disconnect().await;
    }

    pub async fn serve<T, F>(
        self,
        api_connector: F,
        interval_duration: tokio::time::Duration,
    ) -> Result<(), ExternalApiError>
    where
        T: Clone + std::fmt::Debug,
        F: ExternalApiIntegration<T> + Clone + std::fmt::Debug,
    {
        let mut interval = tokio::time::interval(interval_duration);

        loop {
            interval.tick().await;
            if let Err(error) = self
                .clone()
                .import_external_data(api_connector.clone())
                .await
            {
                eprintln!("Error in periodic task: {:?}", error);
            }
        }
    }
}
