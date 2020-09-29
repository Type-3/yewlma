use serde::{Deserialize, Serialize};
use std::fmt;

use super::BreakPoint;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Narrow(pub Option<BreakPoint>);

impl fmt::Display for Narrow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(point) = &self.0 {
            write!(f, "narrow-{}", point)
        } else {
            write!(f, "narrow")
        }
    }
}
