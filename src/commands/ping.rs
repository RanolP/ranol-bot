use bot_any::message::MessageWrite;
use kal::{lex::TransformHintProvider, Command};

#[derive(Command, TransformHintProvider)]
#[command(name = "ping", description = "Pong!")]
pub struct Ping {}

impl Ping {
    pub async fn execute(&self) -> MessageWrite {
        MessageWrite::new().push_text("Pong!".to_string())
    }
}
