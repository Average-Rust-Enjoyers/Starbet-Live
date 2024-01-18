use askama::Template;

use crate::filters;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "login/page.html")]
pub struct LoginPage {}

#[derive(Template)]
#[template(path = "login/form.html")]
pub struct LoginForm {}

#[derive(Template)]
#[template(path = "register/page.html")]
pub struct RegisterPage<'a> {
    pub form: RegisterForm<'a>,
}

#[derive(Template)]
#[template(path = "register/form.html")]
pub struct RegisterForm<'a> {
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

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct Dashboard {
    pub user: UserSend,
    pub menu: Menu,
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
    pub active: bool,
}

#[derive(Template)]
#[template(path = "game.html")]
pub struct Game {
    pub matches: Vec<String>, // TODO: Change this to a struct
    pub game_name: String,
}
