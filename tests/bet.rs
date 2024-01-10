#[cfg(test)]
pub mod bet_tests {
    use chrono::DateTime;
    use sqlx::PgPool;
    use starbet_live::{
        common::repository::{DbReadMany, DbRepository, PoolHandler},
        error::DbResultSingle,
        models::{game::{Game, GameFilter, GameGenre}, bet::{Bet, BetStatus, BetGetByUserId, BetGetById}, game_match_outcome::GameMatchOutcome},
        DbPoolHandler, GameRepository, repositories::bet::BetRepository, DbReadOne,
    };
    use std::sync::Arc;
    use uuid::Uuid;

    #[sqlx::test(fixtures("appuser", "game", "gamematch", "bet"))]
    async fn read_many(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let mut bet_repository = BetRepository::new(PoolHandler::new(arc_pool));
        let app_user_id = Uuid::parse_str("8c5f2846-edd1-461f-a477-ba38583a31f7").unwrap();

        let mut canceled_bet = Bet {
            id: Uuid::parse_str("4b852c26-2bbf-421d-9194-cca0f670e3e3").unwrap(),
            app_user_id: app_user_id.clone(),
            game_match_id: Uuid::parse_str("8902ab54-61fd-40e3-808f-55ddfcd89e37").unwrap(),
            amount: 1114i32,
            status: BetStatus::Canceled,
            expected_outcome: GameMatchOutcome::WinB,
            created_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00").unwrap().into(),
            edited_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00").unwrap().into(),
            deleted_at: None,
        };

        let mut lost_bet = Bet {
            id: Uuid::parse_str("3cd44b3d-9b4d-4080-94d4-092811353396").unwrap(),
            app_user_id: app_user_id.clone(),
            game_match_id: Uuid::parse_str("cd2fce3e-7260-48be-9da1-b85df105e40a").unwrap(),
            amount: 5770i32,
            status: BetStatus::Lost,
            expected_outcome: GameMatchOutcome::WinA,
            created_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00").unwrap().into(),
            edited_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00").unwrap().into(),
            deleted_at: None,
        };

        let read_one = bet_repository
            .read_one(&BetGetById {
                id: Uuid::parse_str("4b852c26-2bbf-421d-9194-cca0f670e3e3").unwrap()
            }).await.expect("The repository call should succeed - read one bet");

        canceled_bet.created_at = read_one.created_at;
        canceled_bet.edited_at = read_one.edited_at;

        assert!(read_one.eq(&canceled_bet));


        let read_one = bet_repository
            .read_one(&BetGetById {
                id: Uuid::parse_str("3cd44b3d-9b4d-4080-94d4-092811353396").unwrap()
            }).await.expect("The repository call should succeed - read one bet");

        lost_bet.created_at = read_one.created_at;
        lost_bet.edited_at = read_one.edited_at;
        assert!(read_one.eq(&lost_bet));

        let by_uid = bet_repository
            .read_many(&BetGetByUserId { user_id: app_user_id })
            .await
            .expect("The repository call should succeed - select by user ID");
        
        println!("{:?} {:?}", by_uid, vec![&lost_bet, &canceled_bet]);
        assert!(by_uid
            .iter()
            .eq(vec![&canceled_bet, &lost_bet]));

            
        bet_repository.disconnect().await;
        Ok(())
    }
}
