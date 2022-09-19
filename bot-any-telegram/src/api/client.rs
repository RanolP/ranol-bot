use apid_telegram_bot::{
    calls::{GetUpdates, SendMessage},
    types::{ChatId, Message, Update},
};
use bot_any::message::{MessageEntity, MessageWrite};
use reqores::ClientRequest;

use crate::bridge::telegram_client_request::TelegramClientRequest;

pub struct TelegramClient<'a> {
    token: &'a str,
}

impl<'a> TelegramClient<'a> {
    pub fn new(token: &'a str) -> Self {
        Self { token }
    }

    pub fn get_updates(&self, offset: Option<i32>) -> impl ClientRequest<Response = Vec<Update>> {
        TelegramClientRequest {
            url: format!("https://api.telegram.org/bot{}/getUpdates", self.token),
            call: GetUpdates {
                offset,
                limit: None,
                timeout: None,
                allowed_updates: None,
            },
        }
    }

    pub fn send_message(
        &self,
        chat_id: ChatId,
        message: MessageWrite,
    ) -> impl ClientRequest<Response = Message> {
        let text: String = message
            .entities
            .into_iter()
            .map(|entity| match entity {
                MessageEntity::StyledText(styled_text) => styled_text.content,
            })
            .collect();

        TelegramClientRequest {
            url: format!("https://api.telegram.org/bot{}/sendMessage", self.token),
            call: SendMessage {
                chat_id,
                text,
                parse_mode: None,
            },
        }
    }
}
