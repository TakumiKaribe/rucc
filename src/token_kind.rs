#[derive(Debug)]
pub(crate) enum TokenKind {
    Reserved(String),
    Num(u32),
    EOF,
}
