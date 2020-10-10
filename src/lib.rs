#![recursion_limit = "512"]
#![feature(bool_to_option)]
#![feature(associated_type_bounds)]

pub mod classes;
#[cfg(feature = "yew")]
pub mod components;
#[cfg(feature = "yew")]
pub mod elements;
#[cfg(feature = "yew")]
pub mod forms;
#[cfg(feature = "yew")]
pub mod layout;
#[cfg(feature = "yew")]
pub mod toast;
pub mod utils;

pub mod prelude {
    pub use super::classes::*;
    #[cfg(feature = "yew")]
    pub use super::components::*;
    #[cfg(feature = "yew")]
    pub use super::elements::*;
    #[cfg(feature = "yew")]
    pub use super::forms::*;
    #[cfg(feature = "yew")]
    pub use super::layout::*;
    #[cfg(feature = "yew")]
    pub use super::toast::*;
}
