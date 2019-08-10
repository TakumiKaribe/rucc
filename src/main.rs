use std::io;

mod node;
mod node_kind;
mod token;
mod token_kind;

use node::Node;
use node_kind::NodeKind;
use token::Token;
use token_kind::*;

fn main() {
    let mut program = String::new();
    io::stdin()
        .read_line(&mut program)
        .unwrap_or_else(|e| panic!("{}", e));

    let mut program = program.chars().peekable();
    let mut tokens: Vec<Token> = vec![];

    let mut num = 0;
    let mut loc = Location { at: 0, len: 0 };

    loop {
        match program.peek() {
            Some(' ') | Some('\n') => {
                program.next();
                loc.succ(1);
            }
            Some(op) if op == &'+' || op == &'-' => {
                loc.len(1);
                tokens.push(Token::new(
                    TokenKind::Reserved(op.to_string(), loc),
                    Some(op.to_string()),
                ));
                program.next();
                loc.succ(1);
            }
            Some(n) if n.is_digit(10) => {
                let mut n = program.next().unwrap();
                loop {
                    num *= 10;
                    num += n.to_digit(10).unwrap();
                    if program.peek().map_or(false, |t| t.is_digit(10)) {
                        n = program.next().unwrap();
                    } else {
                        let digit = f64::from(num).log10() as u32 + 1;
                        loc.len(digit);
                        tokens.push(Token::new(TokenKind::Num(num, loc), Some(num.to_string())));
                        loc.succ(digit);
                        num = 0;
                        break;
                    }
                }
            }
            Some(c) => error_at(*c, loc),
            None => {
                tokens.push(Token::new(TokenKind::EOF, None));
                break;
            }
        }
    }

    dbg!(&tokens);

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    let mut iter = tokens.iter().peekable();
    println!("  mov rax, {}", iter.next().unwrap().expect_number());

    while iter.peek().map_or(false, |t| !t.is_eof()) {
        match iter.next().unwrap() {
            token if token.consume("+") => {
                let token = iter.next().expect("invalid");
                println!("  add rax, {}", token.expect_number());
            }
            token if token.consume("-") => {
                let token = iter.next().expect("invalid");
                println!("  sub rax, {}", token.expect_number());
            }
            _ => panic!("iterate tokens failure"),
        }
    }

    println!("  ret");
}

fn error_at(c: char, loc: Location) {
    (0..loc.at).for_each(|_| print!(" "));
    print!("^");
    println!(" invalid charactor is '{}'", c);
    panic!();
}

fn expr(tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>) -> Box<Node> {
    let mut node = mul(tokens);

    loop {
        let token = tokens.peek().expect("token is none");

        if token.consume("+") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::Add,
                lhs: Some(node),
                rhs: Some(mul(tokens)),
            });
        } else if token.consume("-") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::Sub,
                lhs: Some(node),
                rhs: Some(mul(tokens)),
            });
        } else {
            break;
        }
    }

    node
}

fn mul(tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>) -> Box<Node> {
    let mut node = term(tokens);

    loop {
        let token = tokens.peek().expect("token is none");

        if token.consume("*") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::Mul,
                lhs: Some(node),
                rhs: Some(term(tokens)),
            });
        } else if token.consume("/") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::Div,
                lhs: Some(node),
                rhs: Some(term(tokens)),
            });
        } else {
            break;
        }
    }

    node
}

fn term(tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>) -> Box<Node> {
    let token = tokens.next().expect("token is none");

    if token.consume("(") {
        let node = expr(tokens);
        tokens.next().expect("tokens is none").expect(")");
        node
    } else {
        Box::new(Node {
            kind: NodeKind::Num(token.expect_number()),
            lhs: None,
            rhs: None,
        })
    }
}

fn gen(node: Box<Node>) {
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
