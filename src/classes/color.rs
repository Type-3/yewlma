use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Color {
    #[display(fmt = "white")]
    White,
    #[display(fmt = "black")]
    Black,
    #[display(fmt = "light")]
    Light,
    #[display(fmt = "dark")]
    Dark,
    #[display(fmt = "primary")]
    Primary,
    #[display(fmt = "link")]
    Link,
    #[display(fmt = "danger")]
    Danger,
    #[display(fmt = "success")]
    Success,
    #[display(fmt = "info")]
    Info,
    #[display(fmt = "warning")]
    Warning,
}

impl Color {
    pub fn all() -> &'static [Color] {
        &[
            Color::White,
            Color::Black,
            Color::Light,
            Color::Dark,
            Color::Primary,
            Color::Link,
            Color::Danger,
            Color::Success,
            Color::Info,
            Color::Warning,
        ]
    }
}
