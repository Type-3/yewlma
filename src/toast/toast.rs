use crate::classes::Color;

#[derive(Debug, PartialEq, Clone)]
pub struct Toast {
    pub message: String,
    pub color: Option<Color>,
    pub timeout: Option<u32>,
}

impl Toast {
    pub fn new<S: Into<String>>(msg: S) -> Toast {
        Toast {
            message: msg.into(),
            color: None,
            timeout: None,
        }
    }

    pub fn error<S: Into<String>>(s: S) -> Toast {
        Toast {
            message: s.into(),
            color: Some(Color::Danger),
            timeout: None,
        }
    }

    pub fn success<S: Into<String>>(s: S) -> Toast {
        Toast {
            message: s.into(),
            color: Some(Color::Success),
            timeout: None,
        }
    }

    pub fn timeout(mut self, timeout: u32) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}
