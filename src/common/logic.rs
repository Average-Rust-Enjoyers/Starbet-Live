use sqlx::{Postgres, Transaction};

use crate::{
    common::error::DbError,
    models::{
        bet::{BetGetByMatchId, BetStatus, BetUpdate},
        game_match::GameMatch,
        game_match_outcome::GameMatchOutcome,
        odds::OddsGetByMatchId,
        user::UserUpdateBalance,
    },
    repositories::{bet::BetRepository, odds::OddsRepository, user::UserRepository},
};

pub async fn pay_out_match<'a>(
    game_match: &GameMatch,
    tx: &mut Transaction<'a, Postgres>,
) -> Result<(), DbError> {
    let bets = BetRepository::get_bets_for_game(
        BetGetByMatchId {
            match_id: game_match.id,
        },
        tx,
    )
    .await?;

    if let Some(outcome) = &game_match.outcome {
        for bet in bets {
            if bet.status != BetStatus::Pending {
                continue;
            }

            let mut bet_status = BetStatus::Lost;
            if outcome == &bet.expected_outcome {
                bet_status = BetStatus::Won;

                // TODO: this should be pulled via odds_id from bets once it's added
                let odds = OddsRepository::get_latest_odds_for_match(
                    OddsGetByMatchId {
                        match_id: bet.game_match_id,
                    },
                    tx,
                )
                .await?;

                if let Some(odds) = odds {
                    let multiplier = match outcome {
                        GameMatchOutcome::Draw => 1f64,
                        GameMatchOutcome::WinA => odds.odds_a,
                        GameMatchOutcome::WinB => odds.odds_b,
                    };

                    UserRepository::update_user_balance(
                        UserUpdateBalance {
                            id: bet.app_user_id,
                            delta: (bet.amount as f64 * multiplier).round() as i32,
                        },
                        tx,
                    )
                    .await?;
                }
            }

            BetRepository::update_bet(
                BetUpdate {
                    id: bet.id,
                    status: bet_status,
                },
                tx,
            )
            .await?;

            // TODO: notify websocket regardless of outcome
        }
    }

    Ok(())
}
