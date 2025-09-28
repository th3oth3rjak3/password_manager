use crate::{Message, traits::PageView};
use iced::widget::{column, text};

#[derive(Debug, Default, Clone)]
pub struct LoginPage {}

impl PageView for LoginPage {
    fn view(&'_ self) -> iced::Element<'_, crate::Message> {
        column![text("login page"),].into()
    }

    fn update(&mut self, message: &crate::Message) -> Option<Message> {
        _ = message;
        None
    }
}
