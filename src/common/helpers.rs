use chrono::{DateTime, Datelike, Timelike, Utc};

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
