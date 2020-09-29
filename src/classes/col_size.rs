use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum ColSize {
    #[display(fmt = "one-fifth")]
    OneFifth,
    #[display(fmt = "one-quarter")]
    OneQuarter,
    #[display(fmt = "one-third")]
    OneThird,
    #[display(fmt = "two-fifths")]
    TwoFifths,
    #[display(fmt = "half")]
    Half,
    #[display(fmt = "three-fifths")]
    ThreeFifths,
    #[display(fmt = "three-thirds")]
    TwoThirds,
    #[display(fmt = "three-quarters")]
    ThreeQuarters,
    #[display(fmt = "four-fifths")]
    FourFifths,
    #[display(fmt = "full")]
    Full,
    #[display(fmt = "1")]
    One,
    #[display(fmt = "2")]
    Two,
    #[display(fmt = "3")]
    Three,
    #[display(fmt = "4")]
    Four,
    #[display(fmt = "5")]
    Five,
    #[display(fmt = "6")]
    Six,
    #[display(fmt = "7")]
    Seven,
    #[display(fmt = "8")]
    Eight,
    #[display(fmt = "9")]
    Nine,
    #[display(fmt = "10")]
    Ten,
    #[display(fmt = "11")]
    Eleven,
    #[display(fmt = "12")]
    Twelve,
}
