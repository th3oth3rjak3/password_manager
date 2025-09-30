use crate::{
    Message, PasswordManager, notifications::Notification, password_service, traits::PageView,
};
use iced::widget::{column, text, text_input};

#[derive(Debug, Default, Clone)]
pub struct LoginPage {
    password: String,
    notification: Option<Notification>,
}

impl PageView for LoginPage {
    fn view(&'_ self, app_state: &PasswordManager) -> iced::Element<'_, crate::Message> {
        let mut password_field = text_input("Enter Password", &self.password);

        if !app_state.logged_in {
            password_field = password_field
                .on_input(Message::LoginPasswordChanged)
                .on_submit(Message::LoginRequested)
                .secure(true);
        }

        let mut col = column![password_field];

        if let Some(notification) = &self.notification {
            col = col.push(text(&notification.msg).color(notification.color()));
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
                // TODO: create and integrate a password service
                if self.password == "hello" {
                    self.password = "".into();
                    self.notification = Some(Notification::success("Login Successful!", 10));
                    return_value = Some(Message::LoginSuccess)
                } else {
                    self.notification = Some(Notification::error(
                        "Invalid password, please try again",
                        10,
                    ));
                }
            }
            Message::Tick => {
                if let Some(mut notification) = self.notification.take() {
                    if notification.elapsed + 1 == notification.duration {
                        self.notification = None;
                    } else {
                        notification.elapsed += 1;
                        self.notification = Some(notification);
                    }
                }
            }
            _ => {}
        }

        return_value
    }
}
