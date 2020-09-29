use yew::agent::Dispatcher;
use yew::prelude::*;

use crate::classes::Color;

use super::{Toast, ToastAgent};

pub struct ToastService {
    toast: Dispatcher<ToastAgent>,
}

impl ToastService {
    pub fn error<S: Into<String>>(&mut self, msg: S) {
        self.toast(Color::Danger, msg);
    }

    pub fn warning<S: Into<String>>(&mut self, msg: S) {
        self.toast(Color::Warning, msg);
    }

    pub fn info<S: Into<String>>(&mut self, msg: S) {
        self.toast(Color::Info, msg);
    }

    pub fn success<S: Into<String>>(&mut self, msg: S) {
        self.toast(Color::Success, msg);
    }

    pub fn toast<S: Into<String>>(&mut self, color: Color, msg: S) {
        let toast = Toast::new(msg).color(color);
        self.toast.send(toast);
    }
}

impl Default for ToastService {
    fn default() -> ToastService {
        ToastService {
            toast: ToastAgent::dispatcher(),
        }
    }
}
