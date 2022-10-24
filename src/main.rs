use std::env;

use bot_any_telegram::{
    api::TelegramClient,
    types::{ChatId, MessageContent, UpdateEvent},
};
use dotenvy::dotenv;
use futures_lite::StreamExt;
use kal::{
    lex::{remove_leading, remove_trailing, CommandLexer, TokenTransformer, TransformHintProvider},
    Command, CommandParseError,
};
use ranol_bot::commands::RootCommand;
use reqores_client_surf::SurfClient;
use tracing_unwrap::ResultExt;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let transformer = TokenTransformer::command_group(
        |s| remove_leading("/", s).map(|s| remove_trailing("@ranol_bot", s).unwrap_or(s)),
        RootCommand::hint(),
    );

    let reqores_client = SurfClient::new();
    let token = env::var("TELEGRAM_BOT_TOKEN")
        .map_err(|e| miette::miette!("{}: TELEGRAM_BOT_TOKEN", e))
        .unwrap_or_log();
    let telegram_client = TelegramClient::new(&token);
    let mut long_polling = telegram_client.long_polling(|req| reqores_client.call(req));

    while let Some(event) = long_polling.next().await {
        let event = match event {
            Ok(event) => event,
            Err(e) => {
                tracing::error!("{:?}", e);
                continue;
            }
        };
        tracing::debug!("{:?}", &event);

        match event {
            UpdateEvent::Message { message } => match message.content {
                MessageContent::Text { text, entities: _ } => {
                    let lexer = CommandLexer::new(&text);
                    let fragments: Result<Vec<_>, _> = transformer.transform(lexer).collect();
                    let command = fragments
                        .as_ref()
                        .map_err(CommandParseError::from)
                        .and_then(|fragments| RootCommand::parse(fragments));

                    match command {
                        Ok(RootCommand::Ping(ping)) => {
                            let message_write = ping.execute().await;
                            reqores_client
                                .call(
                                    telegram_client
                                        .send_message(ChatId::Int(message.chat.id), message_write),
                                )
                                .await
                                .unwrap_or_log();
                        }
                        _ => {}
                    }
                }
                content => {
                    dbg!(content);
                }
            },
            e => {
                dbg!(e);
                // do not support now
            }
        }
    }
}
