use askama::Template;
use uuid::Uuid;

use crate::{
    filters,
    models::game_match::GameMatchStatus,
    models::{odds::Odds, user::User},
};

use crate::models;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "admin/index.html")]
pub struct AdminPanel {
    pub games: Vec<models::game::Game>,
    pub matches: Vec<models::game_match::GameMatch>,
}

#[derive(Template)]
#[template(path = "admin/match.html")]
pub struct AdminPanelMatch {
    pub game_match: models::game_match::GameMatch,
}

#[derive(Template)]
#[template(path = "server_error.html")]
pub struct ServerErrorPage {}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginPage {}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterPage<'a> {
    pub username: TextField<'a>,
    pub first_name: TextField<'a>,
    pub last_name: TextField<'a>,
    pub email: TextField<'a>,
    pub password: TextField<'a>,
    pub confirm_password: TextField<'a>,
}

#[derive(Template, Clone)]
#[template(path = "components/textfield.html")]
pub struct TextField<'a> {
    pub name: &'a str,
    pub value: String,
    pub error_message: String,
}

impl TextField<'_> {
    pub fn new(name: &str) -> TextField {
        TextField {
            name,
            value: String::new(),
            error_message: String::new(),
        }
    }
}

#[derive(Template)]
#[template(path = "profile/index.html")]
pub struct ProfilePage {
    pub menu: Vec<ProfileMenuItem>,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub profile_picture: String,
    pub balance: i32,
}

impl From<User> for ProfilePage {
    fn from(user: User) -> Self {
        Self {
            menu: vec![
                ProfileMenuItem::new("bet-history", false, ""),
                ProfileMenuItem::new("edit-profile", false, ""),
                ProfileMenuItem::new("deposit-withdrawal", false, ""),
                ProfileMenuItem::new("settings", false, "mt-auto"),
            ],
            username: user.username,
            email: user.email,
            first_name: user.name,
            last_name: user.surname,
            profile_picture: user.profile_picture,
            balance: user.balance,
        }
    }
}

#[derive(Template)]
#[template(path = "profile/bet_history_bet.html")]
pub struct BetHistoryBet {
    pub game_name: String,
    pub team_a: String,
    pub team_b: String,
    pub predicted_team: String,
    pub bet_amount: i32,
    pub multiplier: f64,
    pub won_amount: i32,
    pub won: bool,
    pub date: String,
}

#[derive(Template)]
#[template(path = "profile/bet_history.html")]
pub struct BetHistory {
    pub menu: Vec<ProfileMenuItem>,
    pub bets: Vec<BetHistoryBet>,
}

impl BetHistory {
    pub fn new(bets: Vec<BetHistoryBet>) -> Self {
        Self {
            menu: vec![
                ProfileMenuItem::new("bet-history", true, ""),
                ProfileMenuItem::new("edit-profile", false, ""),
                ProfileMenuItem::new("deposit-withdrawal", false, ""),
                ProfileMenuItem::new("settings", false, "mt-auto"),
            ],
            bets,
        }
    }
}

#[derive(Template)]
#[template(path = "profile/edit_profile.html")]
pub struct EditProfilePage<'a> {
    pub menu: Vec<ProfileMenuItem>,
    pub username: TextField<'a>,
    pub email: TextField<'a>,
    pub first_name: TextField<'a>,
    pub last_name: TextField<'a>,
}

impl EditProfilePage<'_> {
    pub fn new() -> Self {
        Self {
            menu: vec![
                ProfileMenuItem::new("bet-history", false, ""),
                ProfileMenuItem::new("edit-profile", true, ""),
                ProfileMenuItem::new("deposit-withdrawal", false, ""),
                ProfileMenuItem::new("settings", false, "mt-auto"),
            ],
            username: TextField::new("username"),
            email: TextField::new("email"),
            first_name: TextField::new("first-name"),
            last_name: TextField::new("last-name"),
        }
    }
}

impl Default for EditProfilePage<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Template)]
#[template(path = "profile/info_fragment.html")]
pub struct ProfileInfoFragment {
    pub name: String,
    pub value: String,
}

#[derive(Template)]
#[template(path = "profile/deposit_withdrawal.html")]
pub struct DepositWithdrawalPage {
    pub menu: Vec<ProfileMenuItem>,
}

impl DepositWithdrawalPage {
    pub fn new() -> Self {
        Self {
            menu: vec![
                ProfileMenuItem::new("bet-history", false, ""),
                ProfileMenuItem::new("edit-profile", false, ""),
                ProfileMenuItem::new("deposit-withdrawal", true, ""),
                ProfileMenuItem::new("settings", false, "mt-auto"),
            ],
        }
    }
}

impl Default for DepositWithdrawalPage {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Template)]
#[template(path = "profile/menu_item.html")]
pub struct ProfileMenuItem {
    pub name: String,
    pub active: bool,
    pub custom_classes: String,
}

impl ProfileMenuItem {
    pub fn new(name: &str, active: bool, custom_classes: &str) -> ProfileMenuItem {
        ProfileMenuItem {
            name: name.to_string(),
            active,
            custom_classes: custom_classes.to_owned(),
        }
    }
}

#[derive(Template)]
#[template(path = "profile/balance_fragment.html")]
pub struct ProfileBalanceFragment {
    pub balance: i32,
}

#[derive(Template)]
#[template(path = "profile/settings.html")]
pub struct SettingsPage {
    pub menu: Vec<ProfileMenuItem>,
}

impl SettingsPage {
    pub fn new() -> Self {
        Self {
            menu: vec![
                ProfileMenuItem::new("bet-history", false, ""),
                ProfileMenuItem::new("edit-profile", false, ""),
                ProfileMenuItem::new("deposit-withdrawal", false, ""),
                ProfileMenuItem::new("settings", true, "mt-auto"),
            ],
        }
    }
}

impl Default for SettingsPage {
    fn default() -> Self {
        Self::new()
    }
}

pub struct UserSend {
    pub username: String,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub profile_picture: String,
    pub balance: i32,
}

impl From<&User> for UserSend {
    fn from(user: &User) -> Self {
        Self {
            username: user.username.clone(),
            email: user.email.clone(),
            name: user.name.clone(),
            surname: user.surname.clone(),
            profile_picture: user.profile_picture.clone(),
            balance: user.balance,
        }
    }
}

#[derive(Template)]
#[template(path = "dashboard/dashboard.html")]
pub struct Dashboard {
    pub user: UserSend,
    pub menu: Menu,
    pub active_bets: ActiveBets,
    pub user_nav: UserNav,
}

#[derive(Template)]
#[template(path = "user/user_nav.html")]
pub struct UserNav {
    pub username: String,
    pub user_balance: UserBalance,
}

#[derive(Template)]
#[template(path = "menu/games_menu.html")]
pub struct Menu {
    pub games: Vec<MenuItem>,
}

#[derive(Template)]
#[template(path = "menu/menu_item.html")]
pub struct MenuItem {
    pub name: String,
    pub game_id: Uuid,
    pub active: bool,
}

#[derive(Template)]
#[template(path = "dashboard/game/game.html")]
pub struct Game {
    pub matches: Vec<Match>,
    pub upcoming_matches: Vec<UpcomingMatch>,
    pub game_name: String,
    pub game_id: String,
}

#[derive(Template)]
#[template(path = "dashboard/match/upcoming_match.html")]
pub struct UpcomingMatch {
    pub match_id: Uuid,
    pub team_a: String,
    pub team_b: String,
    pub date: String,
}

#[derive(Template)]
#[template(path = "dashboard/match/match.html")]
pub struct Match {
    pub match_id: Uuid,
    pub team_a: String,
    pub team_b: String,
    pub current_odds: Odds,
}

#[derive(Template)]
#[template(path = "dashboard/match/game_match_wrapper.html")]
pub struct GameMatchWrapper {
    pub match_id: Uuid,
    pub match_template: Match,
}

#[derive(Template)]
#[template(path = "dashboard/bet/place_bet_form.html")]
pub struct PlaceBetForm {
    pub match_id: Uuid,
    pub predicted_team: String,
    pub prediction: String,
}

#[derive(Template)]
#[template(path = "dashboard/bet/bet_card.html")]
pub struct Bet {
    pub game_name: String,
    pub match_id: Uuid,
    pub team_a: String,
    pub team_b: String,
    pub predicted_team: String,
    pub bet_amount: i32,
    pub date: String,
}

#[derive(Template)]
#[template(path = "dashboard/bet/active_bets.html")]
pub struct ActiveBets {
    pub bets: Vec<Bet>,
}

#[derive(Template)]
#[template(path = "user/user_balance.html")]
pub struct UserBalance {
    pub balance: i32,
}
