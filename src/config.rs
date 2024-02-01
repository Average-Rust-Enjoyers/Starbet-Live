pub const POOL_CONNECTIONS: u32 = 10;
pub const SESSION_EXPIRY: time::Duration = time::Duration::hours(2);
pub const API_POLL_INTERVAL: tokio::time::Duration = tokio::time::Duration::from_secs(300);
pub const DEFAULT_ODDS_VALUE: f64 = 1.9;
pub const CLOUDBET_API_GRAPHQL_URL: &str = "https://sports-api-graphql.cloudbet.com/graphql";
