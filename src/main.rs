use std::io;

mod generator;
mod node;
mod node_kind;
mod parser;
mod token;
mod token_kind;
mod tokenizer;

fn main() {
    let mut program = String::new();
    io::stdin()
        .read_line(&mut program)
        .unwrap_or_else(|e| panic!("{}", e));

    let tokens = tokenizer::tokenize(&mut program.chars().peekable());

    dbg!(&tokens);

    let node = parser::expr(&mut tokens.iter().peekable());

    dbg!(&node);

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    generator::gen(node);

    println!("  pop rax");
    println!("  ret");
}
