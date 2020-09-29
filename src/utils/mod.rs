mod null_switch;
pub use self::null_switch::NullSwitchImplementor;

pub trait SwitchImplementor: yew_router::Switch + Clone + 'static {}

impl<T> SwitchImplementor for T where T: yew_router::Switch + Clone + 'static {}
