use crate::{
    common::error::ExternalApiError,
    common::{DbCreateOrUpdate, DbReadAll},
    models::{
        game::Game,
        game_match::{GameMatch, GameMatchCreateOrUpdate},
    },
    repositories::{game::GameRepository, game_match::GameMatchRepository},
    DbRepository, PoolHandler,
};

#[async_trait::async_trait]
pub trait ExternalApiIntegration<T> {
    async fn fetch_game_matches(self, game: &Game) -> Result<Vec<T>, ExternalApiError>;
    fn into(game_match: T, game: &Game) -> Result<GameMatchCreateOrUpdate, ExternalApiError>;
}

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
        for game in games {
            let game_matches: Vec<T> = api.clone().fetch_game_matches(&game).await?;

            let game_matches_extracted_data = game_matches
                .into_iter()
                .map(|game_match| F::into(game_match, &game))
                .collect::<Result<Vec<GameMatchCreateOrUpdate>, ExternalApiError>>()?;

            let mut stored_game_matches = vec![];
            for game_match_data in game_matches_extracted_data {
                let stored_game_match = self
                    .clone()
                    .store_or_update_game_match(game_match_data)
                    .await;
                stored_game_matches.push(stored_game_match);
            }

            stored_game_matches
                .into_iter()
                .collect::<Result<Vec<GameMatch>, ExternalApiError>>()?;
        }

        Ok(())
    }

    async fn store_or_update_game_match(
        &mut self,
        game_match_data: GameMatchCreateOrUpdate,
    ) -> Result<GameMatch, ExternalApiError> {
        self.game_match_repo
            .create_or_update(&game_match_data)
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
