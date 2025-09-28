use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Center, Element, Fill};

use crate::{login_page::LoginPage, traits::PageView};

mod login_page;
mod traits;

#[derive(Debug, Clone)]
pub enum Message {
    NavigateTo(Page),
    OtherThing,
}

#[derive(Default, Debug, Clone)]
pub enum Page {
    #[default]
    Home,
    LoginPage(LoginPage),
}

#[derive(Default)]
pub struct PasswordManager {
    current_page: Page,
}

impl PasswordManager {
    pub fn view(&self) -> Element<'_, Message> {
        let page_content = match &self.current_page {
            Page::Home => self.home_view(),
            Page::LoginPage(login) => login.view(),
        };

        let sidebar = container(
            column![
                button("Home").on_press(Message::NavigateTo(Page::Home)),
                button("Login")
                    .on_press(Message::NavigateTo(Page::LoginPage(LoginPage::default())))
            ]
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
        if let Message::NavigateTo(page) = message {
            self.current_page = page;
            return;
        }

        match &mut self.current_page {
            Page::Home => self.home_update(message),
            Page::LoginPage(login) => {
                if let Some(message) = login.update(&message) {
                    self.home_update(message)
                }
            }
        }
    }

    fn home_view(&self) -> Element<'_, Message> {
        column![text("Home Page"),].into()
    }

    fn home_update(&mut self, message: Message) {
        if let Message::OtherThing = message {
            eprintln!("Other Thing Called");
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
