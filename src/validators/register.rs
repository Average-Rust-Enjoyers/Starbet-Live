use regex::Regex;

pub fn validate_username(username: String) -> (String, String) {
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

pub fn validate_confirm_password(password: &str, confirm_password: String) -> (String, String) {
    if password != confirm_password {
        return (confirm_password, "Passwords do not match".to_string());
    }

    (confirm_password, String::new())
}
