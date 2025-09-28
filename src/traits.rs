use iced::Element;

use crate::Message;

pub trait PageView {
    fn view(&'_ self) -> Element<'_, Message>;
    fn update(&mut self, message: &Message) -> Option<Message>;
}
