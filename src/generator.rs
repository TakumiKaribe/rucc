use super::node::Node;
use super::node_kind::NodeKind::*;

pub(crate) fn gen(node: Box<Node>) {
    if let Num(val) = node.kind {
        println!("  push {}", val);
        return;
    }

    gen(node.lhs.expect("token is none"));
    gen(node.rhs.expect("token is none"));

    println!("  pop rdi");
    println!("  pop rax");

    match node.kind {
        Add => println!("  add rax, rdi"),
        Sub => println!("  sub rax, rdi"),
        Mul => println!("  imul rax, rdi"),
        Div => {
            println!("  cqo");
            println!("  idiv rdi");
        }
        Equal => {
            println!("  cmp rax, rdi");
            println!("  sete al");
            println!("  movzb rax, al");
        }
        NotEqual => {
            println!("  cmp rax, rdi");
            println!("  setne al");
            println!("  movzb rax, al");
        }
        LT => {
            println!("  cmp rax, rdi");
            println!("  setl al");
            println!("  movzb rax, al");
        }
        LTEqual => {
            println!("  cmp rax, rdi");
            println!("  setle al");
            println!("  movzb rax, al");
        }
        GT => {
            println!("  cmp rdi, rax");
            println!("  setl al");
            println!("  movzb rax, al");
        }
        GTEqual => {
            println!("  cmp rdi, rax");
            println!("  setle al");
            println!("  movzb rax, al");
        }
        _ => panic!("invalid kind"),
    }

    println!("  push rax");
}
