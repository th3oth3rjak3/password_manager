use std::path::PathBuf;
use std::sync::OnceLock;

use chrono::{DateTime, Duration, Utc};
use directories::UserDirs;
use iced::border::Radius;
use iced::widget::container::Style;
use iced::widget::{Space, button, column, container, row, scrollable, text};
use iced::{Border, Center, Element, Fill, Subscription, color, theme, time};

pub use services::*;

use crate::messages::Message;
use crate::pages::VaultPage;
use crate::pages::{LoginPage, SetMasterPasswordPage};
use crate::traits::PageView;

mod messages;
mod notifications;
mod pages;
mod services;
mod traits;

static DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

fn main() -> iced::Result {
    if let Some(user_dirs) = UserDirs::new() {
        let home_dir = user_dirs.home_dir();
        let data_dir = home_dir.join(".password_manager");

        std::fs::create_dir_all(&data_dir).expect("could not create application data directory");
        DATA_DIR.set(data_dir.clone()).unwrap();
    } else {
        println!("Could not find user directories. Exiting...");
        std::process::exit(1);
    }

    iced::application(
        "Password Manager",
        PasswordManager::update,
        PasswordManager::view,
    )
    .subscription(PasswordManager::subscription)
    .theme(|_| theme::Theme::CatppuccinMocha)
    .run()
}

#[derive(Debug, Clone)]
pub enum Page {
    LoginPage(LoginPage),
    VaultPage(VaultPage),
    SetMasterPassword(SetMasterPasswordPage),
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
    logged_in: bool,
}

impl PasswordManager {
    pub fn view(&self) -> Element<'_, Message> {
        // TODO: break the layout stuff into its own function

        // TODO: check to see if the password file is loaded or not, if not we need the user to set a master password, so show the master password creation page.

        let page_content = match &self.current_page {
            Page::LoginPage(login) => login.view(self),
            Page::VaultPage(vault) => vault.view(self),
            Page::SetMasterPassword(set_pw) => set_pw.view(self),
        };

        let mut sidebar_column = column![];

        // show other buttons when logged in.
        if self.logged_in {
            sidebar_column = sidebar_column.push(
                button("Vault")
                    .on_press(Message::NavigateTo(Page::VaultPage(VaultPage::default()))),
            );
        }

        sidebar_column = sidebar_column.push(Space::with_height(Fill));

        if self.logged_in {
            sidebar_column = sidebar_column.push(button("Lock").on_press(Message::Logout))
        }

        if let Some(expiry) = self.session_expiry {
            sidebar_column = sidebar_column.push(text("Login Expiration:"));
            sidebar_column = sidebar_column.push(text(format!(
                "{}",
                expiry.with_timezone(&chrono::Local).format("%-I:%M %p")
            )));
        } else {
            sidebar_column = sidebar_column.push(text("Not Logged In"));
        }

        let sidebar = container(
            sidebar_column
                .spacing(10)
                .padding(10)
                .width(200)
                .align_x(Center),
        )
        .height(Fill);

        let content = container(scrollable(page_content).height(Fill)).padding(10);
        let divider = container(column![])
            .height(Fill)
            .width(1)
            .style(container::bordered_box);

        column![row![sidebar, divider, content]].into()
    }

    pub fn update(&mut self, message: Message) {
        match &message {
            Message::NavigateTo(page) => {
                self.current_page = page.clone();
                self.update(Message::RenewExpiration);
            }
            Message::Tick => {
                if let Some(expiry) = self.session_expiry {
                    if Utc::now() >= expiry {
                        self.update(Message::Logout);
                    }
                }
            }
            Message::LoginSuccess => {
                self.logged_in = true;
                self.update(Message::NavigateTo(Page::VaultPage(VaultPage::default())));
            }
            Message::Logout => {
                self.logged_in = false;
                self.session_expiry = None;
                self.update(Message::NavigateTo(Page::LoginPage(LoginPage::default())));
            }
            Message::RenewExpiration => {
                if self.logged_in {
                    self.session_expiry = Some(Utc::now() + Duration::minutes(10));
                }
            }
            _ => {}
        }

        let result = match &mut self.current_page {
            Page::LoginPage(login) => login.update(&message),
            Page::VaultPage(vault) => vault.update(&message),
            Page::SetMasterPassword(set_pw) => set_pw.update(&message),
        };

        if let Some(message) = result {
            self.update(message);
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let ticker = time::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick);

        Subscription::batch([ticker])
    }
}
