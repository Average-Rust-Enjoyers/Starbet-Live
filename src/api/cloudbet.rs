use super::connector::ExternalApiIntegration;
use crate::common::error::ExternalApiError;
use crate::config::CLOUDBET_API_GRAPHQL_URL;
use crate::models::game::Game;
use crate::models::game_match::GameMatchCreateOrUpdate;

#[cynic::schema("cloudbet")]
mod schema {}

#[derive(cynic::QueryVariables, Debug)]
pub struct GameMatchesVariables<'a> {
    pub game_key: &'a str,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "GameMatchesVariables")]
pub struct GameMatches {
    #[arguments(sportKey: $game_key)]
    #[cynic(flatten)]
    pub competitions: Vec<Competition>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Competition {
    pub key: cynic::Id,
    pub name: String,
    #[cynic(flatten, rename = "events")]
    pub game_matches: Vec<GameMatch>,
}

#[derive(Clone, cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Event")]
pub struct GameMatch {
    pub id: cynic::Id,
    pub name: String,
    pub status: GameMatchStatus,
    pub cutoff_time: DateTime,
    pub away: Option<Team>,
    pub home: Option<Team>,
}

#[derive(Clone, cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "TeamIdentifier")]
pub struct Team {
    pub name: String,
    pub nationality: String,
}

#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cynic(graphql_type = "EventStatus")]
pub enum GameMatchStatus {
    PreTrading,
    Trading,
    TradingLive,
    Resulted,
    Interrupted,
    AwaitingResults,
    PostTrading,
    Cancelled,
}

impl From<GameMatchStatus> for crate::models::game_match::GameMatchStatus {
    fn from(cloudbet_value: self::GameMatchStatus) -> Self {
        match cloudbet_value {
            GameMatchStatus::PreTrading => crate::models::game_match::GameMatchStatus::Pending,
            GameMatchStatus::Trading => crate::models::game_match::GameMatchStatus::Pending,

            GameMatchStatus::TradingLive => crate::models::game_match::GameMatchStatus::Live,

            GameMatchStatus::Resulted => crate::models::game_match::GameMatchStatus::Finished,
            GameMatchStatus::Interrupted => crate::models::game_match::GameMatchStatus::Canceled,
            GameMatchStatus::AwaitingResults => {
                crate::models::game_match::GameMatchStatus::AwaitingResults
            }
            GameMatchStatus::PostTrading => crate::models::game_match::GameMatchStatus::Finished,
            GameMatchStatus::Cancelled => crate::models::game_match::GameMatchStatus::Canceled,
        }
    }
}

#[derive(cynic::Scalar, Debug, Clone)]
pub struct DateTime(pub String);

impl From<DateTime> for chrono::DateTime<chrono::Utc> {
    fn from(rfc3339_datetime: DateTime) -> Self {
        chrono::DateTime::parse_from_rfc3339(&rfc3339_datetime.0)
            .unwrap_or_default()
            .with_timezone(&chrono::Utc)
    }
}

#[derive(Clone, Debug)]
pub struct CloudbetApi {
    api_key: String,
}

impl CloudbetApi {
    pub fn new(cloudbet_api_key: String) -> Self {
        Self {
            api_key: cloudbet_api_key,
        }
    }

    fn build_query(
        game_key: &str,
    ) -> cynic::Operation<self::GameMatches, self::GameMatchesVariables> {
        use cynic::QueryBuilder;
        self::GameMatches::build(self::GameMatchesVariables { game_key })
    }

    async fn run_query(self, game_key: &str) -> cynic::GraphQlResponse<self::GameMatches> {
        use cynic::http::ReqwestExt;

        let query = Self::build_query(game_key);

        reqwest::Client::new()
            .post(CLOUDBET_API_GRAPHQL_URL)
            .header("X-API-KEY", self.api_key)
            .run_graphql(query)
            .await
            .unwrap_or(cynic::GraphQlResponse {
                data: None,
                errors: Some(vec![cynic::GraphQlError {
                    message: "Cynic reqwest error. Error fetching game matches from Cloudbet"
                        .to_string(),
                    locations: None,
                    path: None,
                    extensions: None,
                }]),
            })
    }
}

#[async_trait::async_trait]
impl ExternalApiIntegration<self::GameMatch> for CloudbetApi {
    async fn fetch_game_matches(
        self,
        game: &Game,
    ) -> Result<Vec<self::GameMatch>, ExternalApiError> {
        let Some(cloudbet_key) = &game.cloudbet_key.clone() else {
            return Ok(vec![]);
        };

        match self.run_query(cloudbet_key).await {
            cynic::GraphQlResponse {
                data: Some(self::GameMatches { competitions }),
                errors: None,
            } => Ok(competitions
                .into_iter()
                .flat_map(|c| c.game_matches)
                .collect()),
            cynic::GraphQlResponse {
                data: _,
                errors: Some(errors),
            } => Err(ExternalApiError::GraphQl(errors)),
            _ => Err(ExternalApiError::from(
                "Error fetching game matches from Cloudbet",
            )),
        }
    }

    fn into(
        event: self::GameMatch,
        game: &Game,
    ) -> Result<GameMatchCreateOrUpdate, ExternalApiError> {
        let Some(team_a) = event.clone().home else {
            return Err(ExternalApiError::from("No home team"));
        };
        let Some(team_b) = event.away else {
            return Err(ExternalApiError::from("No away team"));
        };

        let ending_time =
            chrono::DateTime::from(event.cutoff_time.clone()) + chrono::Duration::hours(2);

        Ok(GameMatchCreateOrUpdate {
            game_id: game.id,
            cloudbet_id: event.id.into_inner(),
            status: Some(event.status.into()),
            starts_at: Some(event.cutoff_time.clone().into()),
            ends_at: Some(ending_time),
            name_a: Some(team_a.name),
            name_b: Some(team_b.name),
        })
    }
}
