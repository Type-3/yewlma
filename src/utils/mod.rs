mod null_switch;
pub use self::null_switch::NullSwitchImplementor;

mod cdn;
pub use self::cdn::{Cdn, BulmaCdn};

pub trait SwitchImplementor: yew_router::Switch + Clone + 'static {}

impl<T> SwitchImplementor for T where T: yew_router::Switch + Clone + 'static {}
