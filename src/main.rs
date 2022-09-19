use std::{cmp, time::Duration};

use bot_any_telegram::{
    api::TelegramClient,
    types::{ChatId, MessageContent, UpdateEvent},
};
use dotenvy::{dotenv, var};
use kal::{
    lex::{remove_leading, remove_trailing, CommandLexer, TokenTransformer, TransformHintProvider},
    Command, CommandParseError,
};
use miette::IntoDiagnostic;
use ranol_bot::commands::RootCommand;
use reqores_client_surf::SurfClient;
use tokio::time;

#[tokio::main]
async fn main() -> miette::Result<()> {
    dotenv().ok();

    let reqores_client = SurfClient::new();
    let token = var("TELEGRAM_BOT_TOKEN").into_diagnostic()?;
    let telegram_client = TelegramClient::new(&token);

    let mut offset = None;

    let transformer = TokenTransformer::command_group(
        |s| remove_leading("/", s).map(|s| remove_trailing("@ranol_bot", s).unwrap_or(s)),
        RootCommand::hint(),
    );

    loop {
        let updates = reqores_client
            .call(telegram_client.get_updates(offset))
            .await
            .map_err(|e| miette::miette!("{}", e))?;

        for update in updates {
            offset = Some(cmp::max(offset.unwrap_or(i32::MIN), update.update_id + 1));
            let event = if let Some(event) = update.event {
                event
            } else {
                continue;
            };

            match event {
                UpdateEvent::Message { message } => match message.content {
                    MessageContent::Text { text, entities } => {
                        let lexer = CommandLexer::new(&text);
                        let fragments: Result<Vec<_>, _> = transformer.transform(lexer).collect();
                        let fragments = fragments.map_err(|e| miette::miette!("{}", e))?;

                        let command =
                            RootCommand::parse(&fragments).map_err(|e| miette::miette!("{}", e))?;

                        match command {
                            RootCommand::Ping(ping) => {
                                let message_write = ping.execute().await;
                                reqores_client
                                    .call(
                                        telegram_client.send_message(
                                            ChatId::Int(message.chat.id),
                                            message_write,
                                        ),
                                    )
                                    .await
                                    .map_err(|e| miette::miette!("{}", e))?;
                            }
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

    Ok(())
}
