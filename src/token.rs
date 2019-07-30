use super::token_kind::*;

#[derive(Debug)]
pub(crate) struct Token {
    kind: TokenKind,
    raw_string: Option<String>,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, raw_string: Option<String>) -> Token {
        Token { kind, raw_string }
    }

    pub(crate) fn consume(&self, kind: TokenKind) -> bool {
        use TokenKind::*;
        match (&self.kind, kind) {
            (Num(_), Num(_)) | (EOF, EOF) => true,
            (Reserved(ref lhs), Reserved(ref rhs)) if lhs == rhs => true,
            _ => false,
        }
    }
    /*
        pub(crate) fn expect(&self, kind: TokenKind) {
            use TokenKind::*;
            match (&self.kind, &kind) {
                (Num(_), Num(_)) | (EOF, EOF) => {}
                (Reserved(ref lhs), Reserved(ref rhs)) if lhs == rhs => {}
                _ => panic!("'{:?}'ではありません", kind),
            };
        }
    */
    pub(crate) fn expect_number(&self) -> u32 {
        use TokenKind::*;
        match &self.kind {
            Num(n) => *n,
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
