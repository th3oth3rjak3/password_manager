use crate::Page;

#[derive(Debug, Clone)]
pub enum Message {
    // General Messages
    NavigateTo(Page),
    Tick,
    LoginSuccess,
    SessionExpired,
    Logout,
    RenewExpiration,

    // Login Page
    LoginPasswordChanged(String),
    LoginRequested,

    // Vault Page
    VaultSearchChanged(String),

    // Modify Password Page
    SiteChanged(String),
    UsernameChanged(String),
    PasswordChanged(String),
    UpsertPasswordEntry(UpsertPasswordRequest),
}

// TODO: move these models to another file once we have a service
#[derive(Debug, Clone)]
pub struct UpsertPasswordRequest {
    pub site: String,
    pub username: String,
    pub password: String,
}
