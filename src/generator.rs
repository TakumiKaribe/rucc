use super::node::Node;
use super::node_kind::NodeKind;

pub(crate) fn gen(node: Box<Node>) {
    if let NodeKind::Num(val) = node.kind {
        println!("  push {}", val);
        return;
    }

    gen(node.lhs.expect("token is none"));
    gen(node.rhs.expect("token is none"));

    println!("  pop rdi");
    println!("  pop rax");

    match node.kind {
        NodeKind::Add => println!("  add rax, rdi"),
        NodeKind::Sub => println!("  sub rax, rdi"),
        NodeKind::Mul => println!("  imul rax, rdi"),
        NodeKind::Div => {
            println!("  cqo");
            println!("  idiv rdi");
        }
        _ => panic!("invalid kind"),
    }

    println!("  push rax");
}
