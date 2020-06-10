use std::env;
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

    let has_arg = env::var("ARG")
        .ok()
        .map_or(false, |has_arg| has_arg == "true");

    let is_debug = env::var("DEBUG")
        .ok()
        .map_or(false, |is_debug| is_debug == "true");

    if has_arg {
        program = std::env::args()
            .collect::<Vec<String>>()
            .get(1)
            .cloned()
            .unwrap();
    } else {
        io::stdin()
            .read_line(&mut program)
            .unwrap_or_else(|e| panic!("{}", e));
    }

    let mut tokenizer = tokenizer::Tokenizer::new(program);
    let tokens = tokenizer.tokenize();

    if is_debug {
        dbg!(&tokens);
    }

    let mut parser = parser::Parser::new(tokens);
    let program = parser.program();

    if is_debug {
        dbg!(&program);
    }

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
