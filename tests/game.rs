#[cfg(test)]
pub mod game_repo_tests {
    use chrono::DateTime;
    use sqlx::PgPool;
    use starbet_live::{
        common::repository::{DbReadMany, DbRepository, PoolHandler},
        error::DbResultSingle,
        models::game::{Game, GameFilter, GameGenre},
        repositories::game::GameRepository,
        DbPoolHandler,
    };
    use std::sync::Arc;
    use uuid::Uuid;

    #[sqlx::test(fixtures("game"))]
    async fn read_many(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let mut game_repository = GameRepository::new(PoolHandler::new(arc_pool));

        let lol_game = Game {
            id: Uuid::parse_str("b6f1c213-30e5-4cac-9b05-e0a1befbe7ee").unwrap(),
            name: "League of Legends".to_string(),
            cloudbet_key: Some("league-of-legends".to_string()),
            description: "League of Legends is a team-based game with over 140 champions to make epic plays with.".to_string(),
            logo: "https://gaming-cdn.com/images/products/9456/orig/league-of-legends-pc-game-cover.jpg?v=1662363312".to_string(),
            genre: GameGenre::Moba,
            created_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00").unwrap().into(),
            edited_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00").unwrap().into(),
            deleted_at: None,
        };

        let csgo_game = Game {
            id: Uuid::parse_str("ead41a2d-5669-424b-9b55-1a23b1909159").unwrap(),
            name: "Counter-Strike 2".to_string(),
            cloudbet_key: Some("counter-strike".to_string()),
            description: "Counter-Strike 2 expands upon the team-based action gameplay that it pioneered when it was launched 19 years ago.".to_string(),
            logo: "https://cdn.cloudflare.steamstatic.com/steam/apps/730/header.jpg?t=1607019058".to_string(),
            genre: GameGenre::Fps,
            created_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00").unwrap().into(),
            edited_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00").unwrap().into(),
            deleted_at: None,
        };

        let dota2_game = Game {
            id: Uuid::parse_str("1f4642df-3d91-4303-902a-cec021694c13").unwrap(),
            name: "Dota 2".to_string(),
            cloudbet_key: Some("dota-2".to_string()),
            description: "Dota 2 is a multiplayer online battle arena (MOBA) video game developed and published by Valve.".to_string(),
            logo: "https://cdn.cloudflare.steamstatic.com/steam/apps/570/header.jpg?t=1607022750".to_string(),
            genre: GameGenre::Moba,
            created_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00").unwrap().into(),
            edited_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00").unwrap().into(),
            deleted_at: None,
        };

        let valorant_game = Game {
            id: Uuid::parse_str("a3d487f8-2428-447c-af7b-03df0892b6ad").unwrap(),
            name: "Valorant".to_string(),
            cloudbet_key: Some("esport-valorant".to_string()),
            description: "Valorant is a free-to-play multiplayer tactical first-person shooter developed and published by Riot Games.".to_string(),
            logo: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcTFpZfytSFnYi-3IfpNXsgvZKFUIuiA62PLaQ&usqp=CAU".to_string(),
            genre: GameGenre::Fps,
            created_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00").unwrap().into(),
            edited_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00").unwrap().into(),
            deleted_at: None,
        };

        let empty_filter = GameFilter {
            name: None,
            genre: None,
        };
        // correct with no parameters
        let no_params = game_repository
            .read_many(&empty_filter)
            .await
            .expect("The repository call should succeed - no parameters given");

        assert_eq!(
            no_params,
            vec![
                lol_game,
                csgo_game.clone(),
                dota2_game.clone(),
                valorant_game.clone()
            ]
        );

        let filter_by_name = GameFilter {
            name: Some("2".to_string()),
            genre: None,
        };

        let name = game_repository
            .read_many(&filter_by_name)
            .await
            .expect("The repository call should succeed - name parameter given");

        assert_eq!(name, vec![csgo_game.clone(), dota2_game.clone()]);

        let filter_by_genre = GameFilter {
            name: None,
            genre: Some(GameGenre::Fps),
        };

        let genre = game_repository
            .read_many(&filter_by_genre)
            .await
            .expect("The repository call should succeed - genre parameter given");

        assert_eq!(genre, vec![csgo_game.clone(), valorant_game]);

        let filter_by_name_and_genre = GameFilter {
            name: Some("2".to_string()),
            genre: Some(GameGenre::Fps),
        };

        let name_and_genre = game_repository
            .read_many(&filter_by_name_and_genre)
            .await
            .expect("The repository call should succeed - name and genre parameters given");

        assert_eq!(name_and_genre, vec![csgo_game]);

        let filter_by_nonexistent = GameFilter {
            name: Some("nonexistent_string".to_string()),
            genre: Some(GameGenre::Fps),
        };

        let nonexistent = game_repository
            .read_many(&filter_by_nonexistent)
            .await
            .expect("The repository call should succeed - name and genre parameters given");

        assert_eq!(nonexistent, vec![]);

        game_repository.disconnect().await;
        Ok(())
    }
}
