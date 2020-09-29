use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Size {
    #[display(fmt = "small")]
    Small,
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "large")]
    Large,
}
