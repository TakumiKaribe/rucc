use super::token::Token;

pub(crate) struct LVar {
    locals: Vec<Var>,
}

pub(crate) struct Var {
    pub(crate) name: String,
    pub(crate) len: u32,
    pub(crate) offset: u32,
}

impl LVar {
    pub(crate) fn new() -> Self {
        Self { locals: vec![] }
    }

    pub(crate) fn push(&mut self, var: Var) {
        self.locals.push(var);
    }

    pub(crate) fn find_lvar(&self, token: &Token) -> Option<&Var> {
        self.locals.iter().find(|var| var.name == token.raw_string)
    }

    pub(crate) fn offset(&self) -> u32 {
        self.locals.iter().next().map_or(0, |var| var.offset)
    }
}
