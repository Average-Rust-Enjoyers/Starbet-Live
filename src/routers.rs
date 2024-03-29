use axum::{
    http::{HeaderValue, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::{delete, get, patch, post},
    Router,
};

use crate::handlers::{
    self,
    admin::{
        admin_handler, gamematch_random_odds_handler, gamematch_update_handler,
        new_gamematch_handler,
    },
    bet::{get_active_bets_handler, get_bet_handler, place_bet_handler},
    dashboard::dashboard_handler,
    game::game_handler,
    index::index_handler,
    profile::{
        bet_history_handler, delete_profile_handler, deposit_handler, deposit_withdrawal_handler,
        get_edit_profile_handler, post_edit_profile_handler, profile_handler, settings_handler,
        withdrawal_handler,
    },
    register::register_submission_handler,
    user::user_balance_handler,
    ws::{ws_handler, ws_handler_error},
};

pub fn auth_router() -> Router<()> {
    Router::new()
        .route(
            "/login",
            post(handlers::login::post::login).get(handlers::login::get::login),
        )
        .route(
            "/register",
            post(register_submission_handler).get(handlers::register::register_page_handler),
        )
        .route("/logout", get(handlers::login::get::logout))
        .route(
            "/validation/:field",
            post(handlers::validation::validation_handler),
        )
}

pub fn protected_router() -> Router<()> {
    Router::new()
        .route("/dashboard", get(dashboard_handler))
        .route("/games/:game_id", post(game_handler))
        .route("/ws/:game_name", get(ws_handler))
        .route("/bet/:match_id/:prediction", post(place_bet_handler))
        .route("/bet/:match_id/:prediction", get(get_bet_handler))
        .route("/bet/active", get(get_active_bets_handler))
        .route("/user/balance", get(user_balance_handler))
        .route("/user/bet-history", get(bet_history_handler))
        .route("/user/edit-profile", get(get_edit_profile_handler))
        .route("/user/edit-profile", post(post_edit_profile_handler))
        .route("/user/deposit-withdrawal", get(deposit_withdrawal_handler))
        .route("/user/deposit", post(deposit_handler))
        .route("/user/withdrawal", post(withdrawal_handler))
        .route("/user/settings", get(settings_handler))
        .route("/user", delete(delete_profile_handler))
        .route("/profile", get(profile_handler))
        .route("/admin", get(admin_handler))
        .route("/admin/match", post(new_gamematch_handler))
        .route("/admin/match/:id", patch(gamematch_update_handler))
        .route("/ws/error", get(ws_handler_error))
        .route(
            "/admin/match/:id/odds",
            patch(gamematch_random_odds_handler),
        )
}

pub fn public_router() -> Router<()> {
    Router::new().route("/", get(index_handler))
}

/// Redirect using the `HX-Redirect` header.
///
/// Will fail if the supplied Uri contains characters that are not visible ASCII
/// (32-127).
#[derive(Debug, Clone)]
pub struct HxRedirect(pub Uri);

impl IntoResponse for HxRedirect {
    fn into_response(self) -> Response {
        (
            StatusCode::SEE_OTHER,
            [(
                "HX-Redirect",
                HeaderValue::from_maybe_shared(self.0.to_string()).expect("Invalid header value"),
            )],
        )
            .into_response()
    }
}
