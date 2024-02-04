use askama::Template;
use chrono::{DateTime, Datelike, Timelike, Utc};
use uuid::Uuid;

use crate::{
    models::{error::ErrorMessageWS, extension_web_socket::ExtensionWebSocketError},
    templates::ErrorMessage,
};

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

pub fn generate_error_message_template(message: &str, user_id: Uuid) -> ErrorMessageWS {
    ErrorMessageWS {
        app_user_id: user_id,
        message: ErrorMessage {
            message: message.to_string(),
        }
        .render()
        .unwrap(),
    }
}

pub fn show_popup_error<T, E>(
    message: &str,
    error: E,
    id: Uuid,
    error_web_socket: ExtensionWebSocketError,
) -> Result<T, E> {
    let _ = error_web_socket
        .tx
        .send(generate_error_message_template(message, id)); // TODO: async ?
    Err(error)
}
