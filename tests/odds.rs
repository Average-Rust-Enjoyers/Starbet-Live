#[cfg(test)]
pub mod odds_tests {
    use chrono::DateTime;
    use sqlx::PgPool;
    use starbet_live::{
        common::repository::{DbReadMany, DbRepository, PoolHandler},
        error::DbResultSingle,
        models::{
            game_match::GameMatchGetById,
            odds::{Odds, OddsGetById},
        },
        repositories::odds::OddsRepository,
        DbPoolHandler, DbReadOne,
    };
    use std::sync::Arc;
    use uuid::Uuid;

    #[sqlx::test(fixtures("game", "gamematch", "odds"))]
    async fn read_many(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let mut odds_repo = OddsRepository::new(PoolHandler::new(arc_pool));
        let game_match_id = Uuid::parse_str("7e3eb6e6-1cca-46e3-aa12-4c6c887e046a").unwrap();

        let odds_a = Odds {
            id: Uuid::parse_str("ab669b6e-c031-492a-93b0-23b8521b64c0").unwrap(),
            game_match_id,
            odds_a: 1.879,
            odds_b: 1.5316,
            created_at: DateTime::parse_from_rfc3339("2023-04-16 17:22:06+00:00")
                .unwrap()
                .into(),
            deleted_at: None,
        };

        let odds_b = Odds {
            id: Uuid::parse_str("646da0aa-8012-4df2-a562-56602765fed1").unwrap(),
            game_match_id,
            odds_a: 2.7567,
            odds_b: 0.0803,
            created_at: DateTime::parse_from_rfc3339("2023-04-28 07:15:05+00:00")
                .unwrap()
                .into(),
            deleted_at: None,
        };

        let odds_c = Odds {
            id: Uuid::parse_str("19fca48e-37d0-494c-a5a3-ac3ccd29f34c").unwrap(),
            game_match_id,
            odds_a: 0.3545,
            odds_b: 1.7448,
            created_at: DateTime::parse_from_rfc3339("2023-07-23 03:18:33+00:00")
                .unwrap()
                .into(),
            deleted_at: None,
        };

        let read_one = odds_repo
            .read_one(&OddsGetById {
                id: Uuid::parse_str("ab669b6e-c031-492a-93b0-23b8521b64c0").unwrap(),
            })
            .await
            .expect("The repository call should succeed - read one bet");

        assert_eq!(read_one, odds_a.clone());

        let read_one = odds_repo
            .read_one(&OddsGetById {
                id: Uuid::parse_str("646da0aa-8012-4df2-a562-56602765fed1").unwrap(),
            })
            .await
            .expect("The repository call should succeed - read one bet");

        assert_eq!(read_one, odds_b.clone());

        let read_one = odds_repo
            .read_one(&OddsGetById {
                id: Uuid::parse_str("19fca48e-37d0-494c-a5a3-ac3ccd29f34c").unwrap(),
            })
            .await
            .expect("The repository call should succeed - read one bet");

        assert_eq!(read_one, odds_c.clone());

        let by_uid = odds_repo
            .read_many(&GameMatchGetById { id: game_match_id })
            .await
            .expect("The repository call should succeed - select by user ID");

        assert_eq!(by_uid, vec![odds_a, odds_b, odds_c]);

        odds_repo.disconnect().await;
        Ok(())
    }
}
