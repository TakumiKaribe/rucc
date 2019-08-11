use super::token_kind::*;

#[derive(Debug)]
pub(crate) struct Token {
    kind: TokenKind,
    pub(crate) raw_string: String,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, raw_string: String) -> Token {
        Token { kind, raw_string }
    }

    pub(crate) fn consume(&self, op: &str) -> bool {
        use TokenKind::*;
        match &self.kind {
            Reserved(kind, _) if kind == op => true,
            _ => false,
        }
    }

    pub(crate) fn consume_ident(&self) -> bool {
        use TokenKind::*;
        match &self.kind {
            Ident(_, _) => true,
            _ => false,
        }
    }

    pub(crate) fn expect(&self, op: &str) {
        use TokenKind::*;
        match &self.kind {
            Reserved(kind, _) if kind == op => {}
            _ => panic!("{}ではありません", op),
        }
    }

    pub(crate) fn expect_number(&self) -> u32 {
        use TokenKind::*;
        match &self.kind {
            Num(n, _) => *n,
            _ => panic!("数ではありません"),
        }
    }

    pub(crate) fn is_eof(&self) -> bool {
        use TokenKind::*;
        match &self.kind {
            EOF => true,
            _ => false,
        }
    }
}
