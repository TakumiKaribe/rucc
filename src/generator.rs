use crate::node::Node;
use crate::node_kind::NodeKind::*;

fn gen_lval(node: &Node) {
    if let LVar(offset) = node.kind {
        println!("  mov rax, rbp");
        println!("  sub rax, {}", offset);
        println!("  push rax");
        return;
    }
    panic!("left value is not variable")
}

pub(crate) fn gen(node: Box<Node>) {
    match node.kind {
        Num(val) => {
            println!("  push {}", val);
            return;
        }
        LVar(_) => {
            gen_lval(&node);
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            return;
        }
        Assign => {
            gen_lval(&node.lhs.expect("[ASSIGN] left value is not found"));
            gen(node.rhs.expect("[ASSIGN] right value is not found"));
            println!("  pop rdi");
            println!("  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");
            return;
        }
        Return => {
            gen(node.lhs.expect("[RETURN] left value is not found"));
            println!("  pop rax");
            println!("  mov rsp, rbp");
            println!("  pop rbp");
            println!("  ret");
            return;
        }
        _ => {}
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
