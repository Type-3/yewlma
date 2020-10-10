#[cfg(feature = "yew-router")]
mod null_switch;
#[cfg(feature = "yew-router")]
pub use self::null_switch::NullSwitchImplementor;

#[cfg(feature = "yew")]
mod cdn;
#[cfg(feature = "yew")]
pub use self::cdn::{Cdn, BulmaCdn};

#[cfg(feature = "yew-router")]
pub trait SwitchImplementor: yew_router::Switch + Clone + 'static {}

#[cfg(feature = "yew-router")]
impl<T> SwitchImplementor for T where T: yew_router::Switch + Clone + 'static {}
