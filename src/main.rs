use bot_any_telegram::api::TelegramClient;
use dotenvy::{dotenv, var};
use miette::IntoDiagnostic;
use reqores_client_surf::SurfClient;

#[tokio::main]
async fn main() -> miette::Result<()> {
    dotenv().ok();

    let reqores_client = SurfClient::new();
    let token = ""; // var("TELEGRAM_BOT_TOKEN").into_diagnostic()?;
    let telegram_client = TelegramClient::new(&token);

    let updates = reqores_client
        .call(telegram_client.get_updates())
        .await
        .map_err(|e| miette::miette!("{}", e))?;

    dbg!(updates);

    Ok(())
}
