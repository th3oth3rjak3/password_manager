// TODO: a page that prompts the user to set a master password
// TODO: it should have a Set Password field and a Confirm Password Field
// TODO: it should also have a submit button and error checking for sameness. We don't enforce complexity. You're on your own pal.

use crate::traits::PageView;

#[derive(Debug, Clone)]
pub struct SetMasterPasswordPage {}

impl PageView for SetMasterPasswordPage {
    fn view(
        &'_ self,
        app_state: &crate::PasswordManager,
    ) -> iced::Element<'_, crate::messages::Message> {
        todo!()
    }

    fn update(&mut self, message: &crate::messages::Message) -> Option<crate::messages::Message> {
        todo!()
    }
}
