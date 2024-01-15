use askama::Result;
use std::fmt::Display;

// Custom Askama filter
#[allow(clippy::unnecessary_wraps)]
pub fn replace_hyphens<T: Display>(s: T) -> Result<String> {
    let s = s.to_string();
    Ok(s.replace('-', " "))
}
