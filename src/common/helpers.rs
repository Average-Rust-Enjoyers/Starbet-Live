use chrono::{DateTime, Datelike, Timelike, Utc};

use crate::models::user::UserUpdate;

pub fn format_date_time_string_with_seconds(date_time: &DateTime<Utc>) -> String {
    format!(
        "{:02}/{:02}/{} - {:02}:{:02}:{:02}",
        date_time.day(),
        date_time.month(),
        date_time.year(),
        date_time.hour(),
        date_time.minute(),
        date_time.second()
    )
}

pub fn format_date_time_string_without_seconds(date_time: &DateTime<Utc>) -> String {
    format!(
        "{:02}/{:02}/{} - {:02}:{:02}",
        date_time.day(),
        date_time.month(),
        date_time.year(),
        date_time.hour(),
        date_time.minute()
    )
}

pub fn user_update_all_none(userupdate: &UserUpdate) -> bool {
    userupdate.username.is_none()
        && userupdate.email.is_none()
        && userupdate.name.is_none()
        && userupdate.surname.is_none()
        && userupdate.profile_picture.is_none()
        && userupdate.password_hash.is_none()
        && userupdate.balance.is_none()
}
