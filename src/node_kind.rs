#[derive(Debug)]
pub(crate) enum NodeKind {
    Num(u32),
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    LT,
    LTEqual,
    GT,
    GTEqual,
}
