use bot_any::message::MessageWrite;
use kal::{lex::TransformHintProvider, Command};

/// Pong!
#[derive(Command, TransformHintProvider)]
pub struct Ping {}

impl Ping {
    pub async fn execute(&self) -> MessageWrite {
        MessageWrite::new().push_text("Pong!".to_string())
    }
}
