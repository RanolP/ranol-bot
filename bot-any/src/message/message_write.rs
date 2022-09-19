use super::{MessageEntity, StyledText};

#[derive(Default)]
pub struct MessageWrite {
    pub entities: Vec<MessageEntity>,
}

impl MessageWrite {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push_text(mut self, content: String) -> Self {
        self.entities
            .push(MessageEntity::StyledText(StyledText::new(content)));
        self
    }
}
