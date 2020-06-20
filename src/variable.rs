use crate::token::Token;

#[derive(Debug)]
pub(crate) struct Var {
    pub(crate) next: Option<Box<Var>>,
    pub(crate) name: String,
    pub(crate) offset: u32,
}

impl Var {
    pub(crate) fn find_lvar(&self, token: &Token) -> Option<&Var> {
        while let Some(ref var) = &self.next {
            if var.name == token.raw_string && var.name.len() == token.raw_string.len() {
                return Some(&var);
            }
        }
        None
    }
}
