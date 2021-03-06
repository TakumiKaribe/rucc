use std::env;

mod generator;
mod node;
mod node_kind;
mod parser;
mod token;
mod token_kind;
mod tokenizer;
mod variable;

fn main() {
    let is_debug = env::var("DEBUG")
        .ok()
        .map_or(false, |is_debug| is_debug == "true");

    let mut program = std::env::args()
        .collect::<Vec<String>>()
        .get(1)
        .cloned()
        .unwrap();
    if is_debug {
        println!("{}", program);
    }
    program = program[1..program.len() - 1].to_string();
    if is_debug {
        println!("{}", program);
    }

    let mut tokenizer = tokenizer::Tokenizer::new(program);
    let tokens = tokenizer.tokenize();

    if is_debug {
        dbg!(&tokens);
    }

    let mut parser = parser::Parser::new(tokens);
    let ast = parser.program();

    if is_debug {
        dbg!(&ast);
    }

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    ast.iter().for_each(|node| {
        generator::gen(node);
        println!("  pop rax");
    });

    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}
