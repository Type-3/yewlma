use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, PartialEq, Clone, Serialize, Deserialize)]
pub enum BreakPoint {
    #[display(fmt = "mobile")]
    Mobile,
    #[display(fmt = "touch")]
    Touch,
    #[display(fmt = "desktop")]
    Desktop,
    #[display(fmt = "widescreen")]
    WideScreen,
    #[display(fmt = "fullhd")]
    FullHd,
}
