use crate::{
    auth::{self, AuthSession},
    common::{helpers::format_date_time_string_with_seconds, DbUpdateOne},
    error::{AppError, AppResult},
    models::{
        bet::{BetGetByUserId, BetStatus},
        game::GameGetById,
        game_match,
        game_match_outcome::GameMatchOutcome,
        odds,
        user::{UserDelete, UserUpdate},
    },
    repositories::{
        bet::BetRepository, game::GameRepository, game_match::GameMatchRepository,
        odds::OddsRepository, user::UserRepository,
    },
    routers::HxRedirect,
    templates::{
        BetHistory, BetHistoryBet, DepositWithdrawalPage, EditProfilePage, ProfileBalanceFragment,
        ProfileInfoFragment, ProfilePage, SettingsPage,
    },
    DbDelete, DbReadMany, DbReadOne,
};
use askama::Template;
use axum::{
    http::{StatusCode, Uri},
    response::{Html, IntoResponse},
    Extension, Form,
};
use serde::Deserialize;

use super::validation::{validate_and_build, RegisterFormData};

pub async fn profile_handler(auth_session: auth::AuthSession) -> AppResult<Html<String>> {
    let user = auth::is_logged_in(auth_session)?;

    Ok(Html(ProfilePage::from(user).render()?))
}

pub async fn bet_history_handler(
    auth_session: AuthSession,
    Extension(mut bet_repo): Extension<BetRepository>,
    Extension(mut match_repo): Extension<GameMatchRepository>,
    Extension(mut game_repo): Extension<GameRepository>,
    Extension(mut odds_repo): Extension<OddsRepository>,
) -> AppResult<Html<String>> {
    let user = auth::is_logged_in(auth_session)?;

    let user_bets = bet_repo
        .read_many(&BetGetByUserId { user_id: user.id })
        .await?;

    let mut bet_history = Vec::new();

    for bet in &user_bets {
        if bet.status == BetStatus::Pending {
            continue;
        }

        let game_match = match_repo
            .read_one(&game_match::GameMatchGetById {
                id: bet.game_match_id,
            })
            .await?;

        let date = format_date_time_string_with_seconds(&bet.created_at);

        let game_name = game_repo
            .read_one(&GameGetById {
                id: game_match.game_id,
            })
            .await?
            .name;

        let odds = odds_repo
            .read_one(&odds::OddsGetById { id: bet.odds_id })
            .await?;

        let (predicted_team, multiplier, won_amount) = match bet.expected_outcome {
            GameMatchOutcome::WinA => (
                game_match.name_a.clone(),
                odds.odds_a,
                odds.odds_a * f64::from(bet.amount),
            ),
            _ => (
                game_match.name_b.clone(),
                odds.odds_b,
                odds.odds_b * f64::from(bet.amount),
            ),
        };

        #[allow(clippy::cast_possible_truncation)]
        let won_amount = if bet.status == BetStatus::Won {
            won_amount as i32
        } else {
            0
        };

        let bet_history_bet = BetHistoryBet {
            game_name,
            team_a: game_match.name_a,
            team_b: game_match.name_b,
            predicted_team,
            bet_amount: bet.amount,
            multiplier,
            won_amount,
            bet_status: bet.status.clone(),
            date,
        };

        bet_history.push(bet_history_bet);
    }

    Ok(Html(BetHistory::new(bet_history).render()?))
}

pub async fn get_edit_profile_handler(auth_session: AuthSession) -> AppResult<Html<String>> {
    auth::is_logged_in(auth_session)?;

    Ok(Html(EditProfilePage::new().render()?))
}

const FIELDS: [&str; 4] = ["username", "first-name", "last-name", "email"];

pub async fn post_edit_profile_handler(
    auth_session: AuthSession,
    Extension(mut user_repository): Extension<UserRepository>,
    Form(payload): Form<RegisterFormData>,
) -> AppResult<Html<String>> {
    let user = auth::is_logged_in(auth_session)?;

    let mut user_update = UserUpdate::new(
        &user.id,
        Some(&payload.username),
        Some(&payload.email),
        Some(&payload.first_name),
        Some(&payload.last_name),
        None,
        None,
        None,
    );

    let mut response = String::new();

    for field in &FIELDS {
        let (valid, textfield) = validate_and_build(field, &payload, &mut user_repository).await;

        if !valid {
            match *field {
                "username" => user_update.username = None,
                "first-name" => user_update.name = None,
                "last-name" => user_update.surname = None,
                "email" => user_update.email = None,
                _ => {}
            }
            continue;
        }

        response.push_str(
            &ProfileInfoFragment {
                name: (*field).to_string(),
                value: textfield.value,
            }
            .render()?,
        );
    }

    if !user_update.update_fields_none() && (user_repository.update(&user_update).await).is_err() {
        return Err(AppError::StatusCode(StatusCode::INTERNAL_SERVER_ERROR));
    }

    Ok(Html(response))
}

pub async fn deposit_withdrawal_handler(auth_session: AuthSession) -> AppResult<Html<String>> {
    auth::is_logged_in(auth_session)?;

    Ok(Html(DepositWithdrawalPage::new().render()?))
}

#[derive(Deserialize)]
pub struct DepositWithdrawalForm {
    pub amount: i32,
}

pub async fn handle_transaction<F>(
    auth_session: AuthSession,
    Extension(mut user_repository): Extension<UserRepository>,
    Form(payload): Form<DepositWithdrawalForm>,
    transaction: F,
) -> AppResult<Html<String>>
where
    F: Fn(i32, i32) -> Result<i32, StatusCode>,
{
    let user = auth::is_logged_in(auth_session)?;

    let new_balance = transaction(user.balance, payload.amount)?;

    user_repository
        .update(&UserUpdate {
            id: user.id,
            balance: Some(new_balance),
            ..Default::default()
        })
        .await?;

    Ok(Html(
        ProfileBalanceFragment {
            balance: new_balance,
        }
        .render()?,
    ))
}

pub async fn deposit_handler(
    auth_session: AuthSession,
    user_repository: Extension<UserRepository>,
    form: Form<DepositWithdrawalForm>,
) -> AppResult<impl IntoResponse> {
    handle_transaction(auth_session, user_repository, form, |balance, amount| {
        if amount <= 0 {
            Err(StatusCode::BAD_REQUEST)
        } else {
            Ok(balance + amount * 100)
        }
    })
    .await
}

pub async fn withdrawal_handler(
    auth_session: AuthSession,
    user_repository: Extension<UserRepository>,
    form: Form<DepositWithdrawalForm>,
) -> AppResult<impl IntoResponse> {
    handle_transaction(auth_session, user_repository, form, |balance, amount| {
        if amount <= 0 || amount * 100 > balance {
            Err(StatusCode::BAD_REQUEST)
        } else {
            Ok(balance - amount * 100)
        }
    })
    .await
}

pub async fn settings_handler(auth_session: AuthSession) -> AppResult<Html<String>> {
    auth::is_logged_in(auth_session)?;

    Ok(Html(SettingsPage::new().render()?))
}

pub async fn delete_profile_handler(
    auth_session: AuthSession,
    Extension(mut user_repository): Extension<UserRepository>,
) -> AppResult<HxRedirect> {
    let user = auth::is_logged_in(auth_session)?;

    user_repository.delete(&UserDelete::new(&user.id)).await?;

    Ok(HxRedirect(Uri::from_static("/logout")))
}
