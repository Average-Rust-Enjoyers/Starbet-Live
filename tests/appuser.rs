#[cfg(test)]
pub mod appuser_tests {
    use chrono::DateTime;
    use sqlx::PgPool;
    use starbet_live::{
        common::repository::{DbRepository, PoolHandler},
        error::DbResultSingle,
        models::user::{User, UserLogin},
        repositories::user::UserRepository,
        DbPoolHandler, DbReadOne,
    };
    use std::sync::Arc;
    use uuid::Uuid;

    #[sqlx::test(fixtures("appuser"))]
    async fn login_email(pool: PgPool) -> DbResultSingle<()> {
        let arc_pool = Arc::new(pool);

        let mut user_repo = UserRepository::new(PoolHandler::new(arc_pool));
        let user = user_repo
            .read_one(&UserLogin {
                email: "lsherar0@pagesperso-orange.fr".to_string(),
                password: "heslo".to_string(),
            })
            .await
            .expect("user should exist");

        let mut expected_user = User {
            id: Uuid::parse_str("44d873b3-e9b4-4e71-b355-a38d793a861f").unwrap(),
            username: "lsherar0".to_string(),
            email: "lsherar0@pagesperso-orange.fr".to_string(),
            name: "Leslie".to_string(),
            surname: "Sherar".to_string(),
            profile_picture: "https://robohash.org/exvelitperspiciatis.png?size=150x150&set=set1"
                .to_string(),
            password_hash: "$argon2i$v=19$m=16,t=2,p=1$N1FCeUl5ZDZ4ck1GMHEzcA$b/6yLo+OqXOPYA1zmletIg"
                .to_string(),
            created_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00")
                .unwrap()
                .into(),
            edited_at: DateTime::parse_from_rfc3339("2023-12-09 19:38:46.728083+00:00")
                .unwrap()
                .into(),
            deleted_at: None,
            balance: 0,
        };

        assert!(user.created_at == user.edited_at);
        expected_user.created_at = user.created_at;
        expected_user.edited_at = user.created_at;

        assert!(expected_user.eq(&user));

        user_repo
            .read_one(&UserLogin {
                email: "lsherar0@pagesperso-orange.fr".to_string(),
                password: "blbost".to_string(),
            })
            .await
            .expect_err("invalid password should not be accepted");

        user_repo
            .read_one(&UserLogin {
                email: "nope@nope.com".to_string(),
                password: "blbost".to_string(),
            })
            .await
            .expect_err("nonexistent user should not be accepted");

        Ok(())
    }
}
