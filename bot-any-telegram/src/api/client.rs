use apid_telegram_bot::{calls::GetUpdates, types::Update};
use reqores::ClientRequest;

use crate::bridge::telegram_client_request::TelegramClientRequest;

pub struct TelegramClient<'a> {
    token: &'a str,
}

impl<'a> TelegramClient<'a> {
    pub fn new(token: &'a str) -> Self {
        Self { token }
    }

    pub fn get_updates(&self) -> impl ClientRequest<Response = Vec<Update>> {
        TelegramClientRequest {
            url: format!("https://api.telegram.org/bot{}/getUpdates", self.token),
            call: GetUpdates {
                offset: None,
                limit: None,
                timeout: None,
                allowed_updates: None,
            },
        }
    }
}
