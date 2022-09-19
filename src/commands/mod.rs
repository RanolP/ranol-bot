use kal::{command_group, lex::TransformHintProvider};

mod ping;

command_group! {
    #[derive(TransformHintProvider)]
    pub enum RootCommand {
        Ping(ping::Ping)
    }
}
