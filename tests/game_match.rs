#[cfg(test)]
pub mod game_match_tests {
    use std::sync::Arc;

    use chrono::DateTime;
    use sqlx::PgPool;
    use starbet_live::common::{DbReadAll, DbUpdateOne};
    use starbet_live::models::game_match::GameMatchUpdate;
    use starbet_live::models::{game_match::GameMatchStatus, game_match_outcome::GameMatchOutcome};
    use starbet_live::{
        common::DbReadByForeignKey,
        error::DbResultSingle,
        models::game_match::{GameMatch, GameMatchGetById},
        repositories::game_match::GameMatchRepository,
        DbReadOne,
    };
    use starbet_live::{DbPoolHandler, DbRepository, PoolHandler};
    use uuid::Uuid;

    #[sqlx::test(fixtures("game", "gamematch"))]
    async fn read_one(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);
        let mut game_match_repository = GameMatchRepository::new(PoolHandler::new(arc_pool));

        let game_id = Uuid::parse_str("1f4642df-3d91-4303-902a-cec021694c13").unwrap();

        let pending_game_match_id =
            Uuid::parse_str("de9e51a5-4f35-44c6-a9ee-7a3fb2fd1954").unwrap();

        let mut pending_game_match = GameMatch {
            id: pending_game_match_id,
            game_id,
            name_a: "Sialum".to_string(),
            name_b: "Oklahoma City".to_string(),
            starts_at: DateTime::parse_from_rfc3339("2023-09-05 10:29:35+00:00")
                .unwrap()
                .into(),
            ends_at: DateTime::parse_from_rfc3339("2022-11-11 12:03:47+00:00")
                .unwrap()
                .into(),
            status: GameMatchStatus::Pending,
            outcome: Some(GameMatchOutcome::WinA),
            created_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00")
                .unwrap()
                .into(),
            edited_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00")
                .unwrap()
                .into(),
            deleted_at: None,
        };

        let finished_game_match_id =
            Uuid::parse_str("b56c2fb1-6c6c-4931-97b2-525bf3970252").unwrap();
        let mut finished_game_match = GameMatch {
            id: finished_game_match_id,
            game_id: game_id,
            name_a: "Arua".to_string(),
            name_b: "Sorriso".to_string(),
            starts_at: DateTime::parse_from_rfc3339("2023-06-11 02:31:09+00:00")
                .unwrap()
                .into(),
            ends_at: DateTime::parse_from_rfc3339("2022-08-14 04:01:33+00:00")
                .unwrap()
                .into(),
            status: GameMatchStatus::Finished,
            outcome: Some(GameMatchOutcome::Draw),
            created_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00")
                .unwrap()
                .into(),
            edited_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00")
                .unwrap()
                .into(),
            deleted_at: None,
        };

        let read_pending = game_match_repository
            .read_one(&GameMatchGetById {
                id: pending_game_match_id,
            })
            .await
            .expect("The repository call should succeed - read one game_match");

        pending_game_match.created_at = read_pending.created_at;
        pending_game_match.edited_at = read_pending.edited_at;

        assert_eq!(read_pending, pending_game_match);

        let read_finished = game_match_repository
            .read_one(&GameMatchGetById {
                id: finished_game_match_id,
            })
            .await
            .expect("The repository call should succeed - read one game_match");

        finished_game_match.created_at = read_finished.created_at;
        finished_game_match.edited_at = read_finished.edited_at;

        assert_eq!(read_finished, finished_game_match);

        game_match_repository.disconnect().await;
        Ok(())
    }

    #[sqlx::test(fixtures("game", "gamematch"))]
    async fn read_all(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);
        let mut game_match_repository = GameMatchRepository::new(PoolHandler::new(arc_pool));

        let all_game_matches = game_match_repository
            .read_all()
            .await
            .expect("The repository call should succeed - read one game_match");

        assert_eq!(all_game_matches.len(), 17);
        game_match_repository.disconnect().await;
        Ok(())
    }

    #[sqlx::test(fixtures("game", "gamematch"))]
    async fn read_by_foreign_key(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);
        let mut game_match_repository = GameMatchRepository::new(PoolHandler::new(arc_pool));

        let cs2_game_id = Uuid::parse_str("ead41a2d-5669-424b-9b55-1a23b1909159").unwrap();

        let mut live_game_matches_of_cs2 = game_match_repository
            .get_by_foreign_key(&cs2_game_id)
            .await
            .expect("The repository call should succeed - read one game_match");

        let cs2_match1 = GameMatch {
            id: Uuid::parse_str("33b93500-a4ac-46fe-9b11-d3ebe78cbba1").unwrap(),
            game_id: cs2_game_id,
            name_a: "Kamiraba".to_string(),
            name_b: "Stephens Island".to_string(),
            starts_at: DateTime::parse_from_rfc3339("2023-02-23 05:34:16+00:00")
                .unwrap()
                .into(),
            ends_at: DateTime::parse_from_rfc3339("2022-04-01 11:16:39+00:00")
                .unwrap()
                .into(),
            outcome: Some(GameMatchOutcome::WinA),
            status: GameMatchStatus::Live,
            created_at: DateTime::parse_from_rfc3339("2022-04-01 11:16:39+00:00")
                .unwrap()
                .into(),
            edited_at: DateTime::parse_from_rfc3339("2022-04-01 11:16:39+00:00")
                .unwrap()
                .into(),
            deleted_at: None,
        };

        live_game_matches_of_cs2[0].created_at = cs2_match1.created_at;
        live_game_matches_of_cs2[0].edited_at = cs2_match1.edited_at;

        assert_eq!(live_game_matches_of_cs2, vec![cs2_match1]);
        game_match_repository.disconnect().await;
        Ok(())
    }

    #[sqlx::test(fixtures("game", "gamematch"))]
    async fn update_status(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);
        let mut game_match_repository = GameMatchRepository::new(PoolHandler::new(arc_pool));

        let game_match_id = Uuid::parse_str("7e3eb6e6-1cca-46e3-aa12-4c6c887e046a").unwrap();

        let mut game_match = game_match_repository
            .read_one(&GameMatchGetById { id: game_match_id })
            .await
            .expect("The repository call should succeed - read one game_match");

        assert_eq!(game_match.status, GameMatchStatus::Pending);

        let updated_game_match = game_match_repository
            .update(&GameMatchUpdate {
                id: game_match_id,
                name_a: None,
                name_b: None,
                starts_at: None,
                ends_at: None,
                status: Some(GameMatchStatus::Live),
            })
            .await
            .expect("The repository call should succeed - game_match status updated");

        game_match.status = GameMatchStatus::Live;
        game_match.edited_at = updated_game_match.edited_at;
        assert_eq!(game_match, updated_game_match);

        let updated_game_match = game_match_repository
            .update(&GameMatchUpdate {
                id: game_match_id,
                name_a: None,
                name_b: None,
                starts_at: None,
                ends_at: None,
                status: Some(GameMatchStatus::Finished),
            })
            .await
            .expect("The repository call should succeed - game_match status updated");

        game_match.status = GameMatchStatus::Finished;
        game_match.edited_at = updated_game_match.edited_at;
        assert_eq!(game_match, updated_game_match);

        game_match_repository.disconnect().await;

        Ok(())
    }
}
