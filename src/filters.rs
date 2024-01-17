use askama::Result;
use std::fmt::Display;

// Custom Askama filter
#[allow(clippy::unnecessary_wraps)]
pub fn split_and_capitalize<T: Display>(s: T) -> Result<String> {
    Ok(s.to_string()
        .split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first_char) => first_char.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<String>>()
        .join(" "))
}
