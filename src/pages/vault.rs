use iced::widget::text;

use crate::traits::PageView;

#[derive(Debug, Clone, Default)]
pub struct VaultPage {}

impl PageView for VaultPage {
    fn view(&'_ self, app_state: &crate::PasswordManager) -> iced::Element<'_, crate::Message> {
        text("vault page").into()
    }

    fn update(&mut self, message: &crate::Message) -> Option<crate::Message> {
        None
    }
}
