use iced::Element;

use crate::{Message, PasswordManager};

pub trait PageView {
    fn view(&'_ self, app_state: &PasswordManager) -> Element<'_, Message>;
    fn update(&mut self, message: &Message) -> Option<Message>;
}
