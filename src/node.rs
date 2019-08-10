use super::node_kind::NodeKind;

pub(crate) struct Node {
    kind: NodeKind,
    lhs: Box<Node>,
    rhs: Box<Node>,
}

impl Node {
    pub(crate) fn new(kind: NodeKind, lhs: Node, rhs: Node) -> Self {
        Self { kind: kind, lhs: Box::new(lhs), rhs: Box::new(rhs) }
    }
}
