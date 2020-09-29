mod color;
pub use self::color::Color;

mod breakpoint;
pub use self::breakpoint::BreakPoint;

mod col_size;
pub use self::col_size::ColSize;

mod narrow;
pub use self::narrow::Narrow;

mod size;
pub use self::size::Size;

use std::fmt;

pub trait CssRepr {
    fn prefixed(&self, prefix: &str) -> String;

    fn suffixed(&self, suffix: &str) -> String;

    fn is(&self) -> String {
        self.prefixed("is-")
    }

    fn has_bg(&self) -> String {
        self.prefixed("has-background-")
    }

    fn has_text(&self) -> String {
        self.prefixed("has-text-")
    }

    fn is_offset(&self) -> String {
        self.prefixed("is-offset-")
    }
}

impl<T: fmt::Display> CssRepr for Option<T> {
    fn prefixed(&self, prefix: &str) -> String {
        if let Some(item) = self {
            format!("{}{}", prefix, item)
        } else {
            String::new()
        }
    }

    fn suffixed(&self, suffix: &str) -> String {
        if let Some(item) = self {
            format!("{}{}", item, suffix)
        } else {
            String::new()
        }
    }
}
