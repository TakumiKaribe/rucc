mod token;
mod token_kind;

use token::Token;
use token_kind::Location;
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
    let mut loc = Location { at: 0, len: 0 };

    loop {
        match arg.peek() {
            Some(' ') => {
                arg.next();
                loc.succ(1);
            }
            Some(op) if op == &'+' || op == &'-' => {
                loc.len(1);
                tokens.push(Token::new(
                    TokenKind::Reserved(op.to_string(), loc),
                    Some(op.to_string()),
                ));
                arg.next();
                loc.succ(1);
            }
            Some(n) if n.is_digit(10) => {
                let mut n = arg.next().unwrap();
                loop {
                    num *= 10;
                    num += n.to_digit(10).unwrap();
                    if arg.peek().map_or(false, |t| t.is_digit(10)) {
                        n = arg.next().unwrap();
                    } else {
                        tokens.push(Token::new(TokenKind::Num(num, loc), Some(num.to_string())));
                        let digit = f64::from(num).log10() as u32 + 1;
                        loc.succ(digit);
                        loc.len(digit);
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
            token
                if token.consume(TokenKind::Reserved(
                    '+'.to_string(),
                    Location { at: 0, len: 0 },
                )) =>
            {
                let token = iter.next().expect("invalid");
                println!("  add rax, {}", token.expect_number());
            }
            token
                if token.consume(TokenKind::Reserved(
                    '-'.to_string(),
                    Location { at: 0, len: 0 },
                )) =>
            {
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
    (0..loc.len).for_each(|_| print!("^"));
    println!(" invalid charactor is {}", c);
    panic!();
}
