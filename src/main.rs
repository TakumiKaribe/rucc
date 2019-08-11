use std::io;

mod generator;
mod node;
mod node_kind;
mod parser;
mod token;
mod token_kind;
mod tokenizer;
mod variable;

fn main() {
    let mut program = String::new();
    io::stdin()
        .read_line(&mut program)
        .unwrap_or_else(|e| panic!("{}", e));

    let tokens = tokenizer::tokenize(&mut program.chars().peekable());

    dbg!(&tokens);

    let program = parser::program(&mut tokens.iter().peekable());

    dbg!(&program);

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    program.into_iter().for_each(|ast| {
        generator::gen(ast);
        println!("  pop rax");
    });

    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}
