use iced::{Color, color};

#[derive(Debug, Clone, Copy)]
pub enum NotificationType {
    Success,
    Error,
    Info,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub ty: NotificationType,
    pub msg: String,
    pub duration: u64,
    pub elapsed: u64,
}

impl Notification {
    pub fn success(msg: impl Into<String>, duration: u64) -> Self {
        Self {
            ty: NotificationType::Success,
            msg: msg.into(),
            duration,
            elapsed: 0,
        }
    }

    pub fn error(msg: impl Into<String>, duration: u64) -> Self {
        Self {
            ty: NotificationType::Error,
            msg: msg.into(),
            duration,
            elapsed: 0,
        }
    }

    pub fn info(msg: impl Into<String>, duration: u64) -> Self {
        Self {
            ty: NotificationType::Info,
            msg: msg.into(),
            duration,
            elapsed: 0,
        }
    }

    pub fn color(&self) -> Color {
        match self.ty {
            NotificationType::Success => color!(0x53A653),
            NotificationType::Error => color!(0xB00020),
            NotificationType::Info => color!(0xFFFFFF),
        }
    }
}
