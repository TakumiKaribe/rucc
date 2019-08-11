#[derive(Debug)]
pub(crate) enum TokenKind {
    Reserved(String, Location),
    Num(u32, Location),
    Ident(String, Location),
    EOF,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Location {
    pub(crate) at: u32,
    pub(crate) len: u32,
}

impl Location {
    pub(crate) fn succ(&mut self, n: u32) {
        self.at += n;
    }
    pub(crate) fn len(&mut self, len: u32) {
        self.len = len;
    }
}
