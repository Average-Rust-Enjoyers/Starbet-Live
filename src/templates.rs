use askama::Template;

#[derive(Template)]
#[template(path = "landing_page.html")]
pub struct LandingPageTemplate {}

#[derive(Template)]
#[template(path = "login/page.html")]
pub struct LoginPageTemplate {}

#[derive(Template)]
#[template(path = "login/form.html")]
pub struct LoginFormTemplate {}

#[derive(Template)]
#[template(path = "register/page.html")]
pub struct RegisterPageTemplate {}

#[derive(Template)]
#[template(path = "register/form.html")]
pub struct RegisterFormTemplate {}

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
pub struct DashboardTemplate {
    pub items: Vec<String>,
    pub user: UserSend,
}

#[derive(Template)]
#[template(path = "game.html")]
pub struct GameTemplate {
    pub matches: Vec<String>, // TODO: Change this to a struct
    pub game_name: String,
}
