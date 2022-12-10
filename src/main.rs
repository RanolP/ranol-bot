use std::{env, net::SocketAddr};

use axum::{routing::get, Json, Router};
use bot_any_telegram::{
    api::TelegramClient,
    types::{ChatId, MessageContent, Update, UpdateEvent},
};
use dotenvy::dotenv;
use futures_lite::StreamExt;
use kal::{
    lex::{remove_leading, remove_trailing, CommandLexer, TokenTransformer, TransformHintProvider},
    Command, CommandParseError,
};
use ranol_bot::commands::RootCommand;
use reqores_client_surf::SurfClient;
use tokio::join;
use tracing_unwrap::ResultExt;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let token = env::var("TELEGRAM_BOT_TOKEN")
        .map_err(|e| miette::miette!("{}: TELEGRAM_BOT_TOKEN", e))
        .unwrap_or_log();

    let port: u16 = env::var("PORT")
        .map_err(|e| miette::miette!("{}: PORT", e))
        .unwrap_or_log()
        .parse()
        .unwrap_or_log();

    let transformer = TokenTransformer::command_group(
        |s| remove_leading("/", s).map(|s| remove_trailing("@ranol_bot", s).unwrap_or(s)),
        RootCommand::hint(),
    );

    let reqores_client = SurfClient::new();
    let reqores_client = &reqores_client;
    let telegram_client = TelegramClient::new(&token);
    let telegram_client = &telegram_client;

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let tx_axum_path = tx.clone();
    let test = move |Json(update): Json<Update>| async move {
        if let Some(event) = update.event {
            tx_axum_path.send(event).unwrap_or_log()
        }
    };

    let router = Router::new().route("/telegram", get(test));
    let socket_addr = SocketAddr::from(([0, 0, 0, 0], port));
    let server_fut = axum::Server::bind(&socket_addr).serve(router.into_make_service());

    let long_polling_fut = (move || async move {
        let mut long_polling = telegram_client.long_polling(|req| reqores_client.call(req));

        while let Some(event) = long_polling.next().await {
            match event {
                Ok(event) => tx.send(event).unwrap_or_log(),
                Err(e) => tracing::error!("{:?}", e),
            }
        }
    })();

    let axum_req_fut = (move || async move {
        while let Some(event) = rx.recv().await {
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
                                        telegram_client.send_message(
                                            ChatId::Int(message.chat.id),
                                            message_write,
                                        ),
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
    })();

    let (server, _, _) = join!(server_fut, long_polling_fut, axum_req_fut);

    server.unwrap_or_log();
}
