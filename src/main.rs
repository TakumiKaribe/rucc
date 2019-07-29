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
        if let Some(' ') = arg.peek() {
            arg.next();
            continue;
        }

        match arg.peek() {
            Some('+') | Some('-') => {
                let op = arg.next().unwrap();
                tokens.push(Token::new(
                    TokenKind::Reserved(op.to_string()),
                    Some(op.to_string()),
                ));
                continue;
            }

            None => {
                tokens.push(Token::new(TokenKind::EOF, None));
                break;
            }

            _ => {}
        }

        loop {
            if arg.peek().map_or(false, |v| v.is_digit(10)) {
                num *= 10;
                num += arg.next().unwrap().to_digit(10).unwrap();
            } else {
                tokens.push(Token::new(TokenKind::Num(num), Some(num.to_string())));
                num = 0;
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
