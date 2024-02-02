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
