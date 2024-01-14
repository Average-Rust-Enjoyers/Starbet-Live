use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RegisterFormData {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub password_confirm: String,
}

pub fn validate_username(username: String) -> (String, String) {
    if username.is_empty() {
        return (username, "Username cannot be empty".to_string());
    }
    if username.len() < 3 {
        return (
            username,
            "Username must be at least 3 characters long".to_string(),
        );
    }
    if username.len() > 20 {
        return (
            username,
            "Username must be at most 20 characters long".to_string(),
        );
    }
    if !username
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return (
            username,
            "Username must contain only alphanumeric characters and underscores".to_string(),
        );
    }

    //TODO: check if username is already taken
    (username, String::new())
}
