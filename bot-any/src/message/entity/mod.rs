pub use styled_text::StyledText;

mod styled_text;

pub enum MessageEntity {
    StyledText(StyledText),
}
