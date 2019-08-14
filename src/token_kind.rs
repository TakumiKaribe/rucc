use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub(crate) enum TokenKind {
    Reserved(String),
    Num(u32),
    Ident(String),
    Return,
    EOF,
}

lazy_static! {
    pub(crate) static ref KEYWORD: HashMap<&'static str, TokenKind> =
        [("return", TokenKind::Return)]
            .iter()
            .cloned()
            .collect::<HashMap<_, _>>();
}
