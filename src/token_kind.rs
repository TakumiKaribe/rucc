#[derive(Debug)]
pub(crate) enum TokenKind {
    Reserved(String),
    Num(u32),
    Ident(String),
    Return,
    EOF,
}

impl TokenKind {
    pub(crate) fn keywords(word: &String) -> Option<Self> {
        match word.as_str() {
            "return" => Some(TokenKind::Return),
            _ => None,
        }
    }
}
