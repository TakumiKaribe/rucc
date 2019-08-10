#[derive(Debug)]
pub(crate) enum NodeKind {
    Add,
    Sub,
    Mul,
    Div,
    Num(u64)
}
