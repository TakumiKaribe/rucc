use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub(crate) enum TokenKind {
    Reserved(String),
    Num(u32),
    Ident(String),
    Return,
    If,
    Else,
    While,
    For,
    EOF,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use TokenKind::*;
        match self {
            Reserved(reserved) => write!(f, "{}", reserved),
            Num(num) => write!(f, "{}", num.to_string()),
            Ident(ident) => write!(f, "{}", ident),
            Return => write!(f, "return"),
            If => write!(f, "if"),
            Else => write!(f, "else"),
            While => write!(f, "while"),
            For => write!(f, "for"),
            EOF => write!(f, "EOF"),
        }
    }
}

lazy_static! {
    pub(crate) static ref KEYWORD: HashMap<&'static str, TokenKind> = [
        ("return", TokenKind::Return),
        ("if", TokenKind::If),
        ("else", TokenKind::Else),
        ("while", TokenKind::While),
        ("for", TokenKind::For)
    ]
    .iter()
    .cloned()
    .collect::<HashMap<_, _>>();
}
