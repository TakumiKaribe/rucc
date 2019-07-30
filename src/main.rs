mod token;
mod token_kind;

use token::Token;
use token_kind::TokenKind;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("invalid arguments");
        std::process::exit(1);
    }

    let mut tokens: Vec<Token> = vec![];
    let mut arg = args.get(1).unwrap().chars().peekable();

    let mut num = 0;
    loop {
        match arg.peek() {
            Some(' ') => {
                arg.next();
            }
            Some(op) if op == &'+' || op == &'-' => {
                tokens.push(Token::new(
                    TokenKind::Reserved(op.to_string()),
                    Some(op.to_string()),
                ));
                arg.next();
            }
            Some(n) if n.is_digit(10) => {
                let mut n = arg.next().unwrap();
                loop {
                    num *= 10;
                    num += n.to_digit(10).unwrap();
                    if arg.peek().map_or(false, |t| t.is_digit(10)) {
                        n = arg.next().unwrap();
                    } else {
                        tokens.push(Token::new(TokenKind::Num(num), Some(num.to_string())));
                        num = 0;
                        break;
                    }
                }
            }
            Some(c) => panic!("invalid charactor {}", c),
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
            token if token.consume(TokenKind::Reserved('+'.to_string())) => {
                let token = iter.next().expect("invalid");
                println!("  add rax, {}", token.expect_number());
            }
            token if token.consume(TokenKind::Reserved('-'.to_string())) => {
                let token = iter.next().expect("invalid");
                println!("  sub rax, {}", token.expect_number());
            }
            _ => panic!("iterate tokens failure"),
        }
    }

    println!("  ret");
}
