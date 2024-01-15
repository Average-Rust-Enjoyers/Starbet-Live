use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterFormData {
    pub username: String,
    #[serde(rename = "first-name")]
    pub first_name: String,
    #[serde(rename = "last-name")]
    pub last_name: String,
    pub email: String,
    pub password: String,
    #[serde(rename = "confirm-password")]
    pub confirm_password: String,
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

pub fn vlaidate_name(name: String) -> (String, String) {
    if name.len() < 2 {
        return (name, "Name must be at least 2 characters long".to_string());
    }
    (name, String::new())
}

pub fn validate_email(email: String) -> (String, String) {
    let email_regex = Regex::new(r"(?i)^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$").unwrap();
    if !email_regex.is_match(&email) {
        return (
            email,
            "Invalid email, please follow this template: user@example.com".to_string(),
        );
    }

    (email, String::new())
}

pub fn validate_password(password: String) -> (String, String) {
    if password.len() < 8 {
        return (
            password,
            "Password must be at least 8 characters long".to_string(),
        );
    }
    if password.len() > 50 {
        return (
            password,
            "Password must be at most 50 characters long".to_string(),
        );
    }
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return (
            password,
            "Password must contain at least one uppercase letter".to_string(),
        );
    }
    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        return (
            password,
            "Password must contain at least one lowercase letter".to_string(),
        );
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return (
            password,
            "Password must contain at least one digit".to_string(),
        );
    }
    if !password
        .chars()
        .any(|c| !c.is_ascii_alphanumeric() && !c.is_ascii_whitespace())
    {
        return (
            password,
            "Password must contain at least one special character".to_string(),
        );
    }

    (password, String::new())
}
