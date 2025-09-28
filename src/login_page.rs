use crate::{Message, PasswordManager, traits::PageView};
use iced::{
    color,
    widget::{column, text, text_input},
};

#[derive(Debug, Default, Clone)]
pub struct LoginPage {
    password: String,
    success_message: Option<String>,
    error_message: Option<String>,
}

impl PageView for LoginPage {
    fn view(&'_ self, app_state: &PasswordManager) -> iced::Element<'_, crate::Message> {
        let mut password_field = text_input("Enter Password", &self.password);

        if let None = app_state.session_expiry {
            password_field = password_field
                .on_input(Message::LoginPasswordChanged)
                .on_submit(Message::LoginRequested)
                .secure(true);
        }

        let mut col = column![password_field];

        if let Some(success) = &self.success_message {
            let success_color = color!(0x53A653);
            col = col.push(text(success).color(success_color))
        }

        if let Some(error) = &self.error_message {
            let error_color = color!(0xB00020);
            col = col.push(text(error).color(error_color))
        }

        col.into()
    }

    fn update(&mut self, message: &crate::Message) -> Option<Message> {
        let mut return_value: Option<Message> = None;

        match message {
            Message::LoginPasswordChanged(pw) => {
                self.password = pw.to_owned();
            }
            Message::LoginRequested => {
                // check the pw to see if the user entered it right, then emit a new message somehow...
                if self.password == "hello" {
                    self.password = "".into();
                    self.error_message = None;
                    self.success_message = Some("Logged in!".into());
                    return_value = Some(Message::LoginSuccess)
                } else {
                    self.success_message = None;
                    self.error_message = Some("Invalid password, please try again".into());
                }
            }
            _ => {}
        }

        return_value
    }
}
