use super::node_kind::NodeKind;

#[derive(Debug)]
pub(crate) struct Node {
    pub(crate) kind: NodeKind,
    pub(crate) lhs: Option<Box<Node>>,
    pub(crate) rhs: Option<Box<Node>>,
}
