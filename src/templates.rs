use askama::Template;

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
pub struct RegisterPage {}

#[derive(Template)]
#[template(path = "register/form.html")]
pub struct RegisterForm {}

#[derive(Template)]
#[template(path = "components/textfield.html")]
pub struct TextField<'a> {
    pub name: &'a str,
    pub value: &'a str,
    pub placeholder: &'a str,
    pub is_valid: bool,
    pub error_message: &'a str,
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
    pub items: Vec<String>,
    pub user: UserSend,
}

#[derive(Template)]
#[template(path = "game.html")]
pub struct Game {
    pub matches: Vec<String>, // TODO: Change this to a struct
    pub game_name: String,
}
