use std::collections::HashSet;

pub struct StyledText {
    pub content: String,
    pub style_set: HashSet<Style>,
}

impl StyledText {
    pub fn new(content: String) -> Self {
        Self {
            content,
            style_set: HashSet::new(),
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
pub enum Style {
    Bold,
    Italic,
    Code,
    Pre,
    TextLink(String),
}
