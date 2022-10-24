use apid_telegram_bot::{
    calls::{GetUpdates, SendMessage},
    types::{ChatId, Message, Update},
};
use bot_any::message::{MessageEntity, MessageWrite};
use futures_lite::Future;
use reqores::ClientRequest;

use crate::bridge::telegram_client_request::TelegramClientRequest;

use super::polling::LongPolling;

pub struct TelegramClient<'a> {
    token: &'a str,
}

impl<'a> TelegramClient<'a> {
    pub fn new(token: &'a str) -> Self {
        Self { token }
    }

    pub fn long_polling<F, Fut, Error>(&'a self, call: F) -> LongPolling<'a, F, Fut, Error>
    where
        F: Fn(TelegramClientRequest<GetUpdates>) -> Fut,
        F: Unpin,
        Fut: Future<Output = Result<Vec<Update>, Error>>,
    {
        LongPolling::new(call, self)
    }

    pub fn get_updates(&self, offset: Option<i32>) -> TelegramClientRequest<GetUpdates> {
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
