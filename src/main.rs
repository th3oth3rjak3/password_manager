use chrono::{DateTime, Duration, Local, Utc};
use iced::widget::{Space, button, column, container, row, scrollable, text};
use iced::{Center, Element, Fill, Length};

use crate::login_page::LoginPage;
use crate::traits::PageView;

mod login_page;
mod traits;

#[derive(Debug, Clone)]
pub enum Message {
    NavigateTo(Page),
    Tick,
    LoginSuccess,
    LoginPasswordChanged(String),
    LoginRequested,
}

#[derive(Debug, Clone)]
pub enum Page {
    LoginPage(LoginPage),
}

impl Default for Page {
    fn default() -> Self {
        Page::LoginPage(LoginPage::default())
    }
}

#[derive(Default)]
pub struct PasswordManager {
    current_page: Page,
    session_expiry: Option<DateTime<Utc>>,
}

impl PasswordManager {
    pub fn view(&self) -> Element<'_, Message> {
        let page_content = match &self.current_page {
            Page::LoginPage(login) => login.view(self),
        };

        // Build column first
        let mut sidebar_column = column![
            button("Login").on_press(Message::NavigateTo(Page::LoginPage(LoginPage::default())))
        ];

        sidebar_column = sidebar_column.push(Space::with_height(Fill));

        // Conditionally render expiration or not logged in
        if let Some(expiry) = self.session_expiry {
            sidebar_column = sidebar_column.push(text("Login Expiration:"));
            sidebar_column = sidebar_column.push(text(format!(
                "{}",
                expiry.with_timezone(&chrono::Local).format("%-I:%M %p")
            )));
        } else {
            sidebar_column = sidebar_column.push(text("Not Logged In"));
        }

        // Now wrap it into container
        let sidebar = container(
            sidebar_column
                .spacing(10)
                .padding(10)
                .width(200)
                .align_x(Center),
        )
        .style(container::rounded_box)
        .height(Fill);

        let content = container(scrollable(page_content).height(Fill)).padding(10);

        column![row![sidebar, content]].into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::NavigateTo(page) => self.current_page = page,
            Message::Tick => {
                todo!("verify auth expiry");
            }
            Message::LoginSuccess => {
                self.session_expiry = Some(Utc::now() + Duration::minutes(5));
            }
            _ => match &mut self.current_page {
                Page::LoginPage(login) => {
                    let result = login.update(&message);
                    if let Some(message) = result {
                        self.update(message);
                    }
                }
            },
        }
    }
}

fn main() -> iced::Result {
    iced::run(
        "Password Manager",
        PasswordManager::update,
        PasswordManager::view,
    )
}
