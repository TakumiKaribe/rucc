use crate::location::Location;
use crate::token::Token;
use crate::token_kind::*;

fn error_at(c: char, loc: Location) {
    (0..loc.at).for_each(|_| print!(" "));
    print!("^");
    println!(" invalid charactor is '{}'", c);
    panic!();
}

pub(crate) fn tokenize(program: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut num = 0;
    let mut loc = Location { at: 0, len: 0 };

    loop {
        match program.peek() {
            Some(' ') | Some('\n') => {
                program.next();
                loc.succ(1);
            }

            Some(op)
                if op == &'+'
                    || op == &'-'
                    || op == &'*'
                    || op == &'/'
                    || op == &'('
                    || op == &')'
                    || op == &';' =>
            {
                loc.len(1);
                tokens.push(Token::new(
                    TokenKind::Reserved(op.to_string()),
                    op.to_string(),
                    loc,
                ));
                program.next();
                loc.succ(1);
            }

            Some(op) if op == &'=' || op == &'<' || op == &'>' => {
                let mut op = program.next().unwrap().to_string();
                loc.len(1);
                if let Some(&'=') = program.peek() {
                    op.push(program.next().unwrap());
                    loc.len(2);
                }
                tokens.push(Token::new(TokenKind::Reserved(op.clone()), op, loc));
                if loc.len == 2 {
                    loc.succ(2);
                } else {
                    loc.succ(1);
                }
            }

            Some(op) if op == &'!' => {
                if let Some(&'=') = program.peek() {
                    loc.len(2);
                    tokens.push(Token::new(
                        TokenKind::Reserved("!=".to_string()),
                        "!=".to_string(),
                        loc,
                    ));
                    loc.succ(2);
                    program.next();
                    program.next();
                } else {
                    error_at(*program.peek().unwrap(), loc);
                }
            }

            Some(c) if ('a'..='z').contains(c) => {
                let mut var = program.next().unwrap().to_string();
                let mut len = 1;
                while program.peek().map_or(false, |c| ('a'..='z').contains(c)) {
                    var.push(program.next().unwrap());
                    len += 1;
                }
                loc.len(len);
                match TokenKind::keywords(&var) {
                    Some(kind) => tokens.push(Token::new(kind, var, loc)),
                    None => tokens.push(Token::new(TokenKind::Ident(var.clone()), var, loc)),
                }
                loc.succ(len);
                program.next();
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
                        tokens.push(Token::new(TokenKind::Num(num), num.to_string(), loc));
                        loc.succ(digit);
                        num = 0;
                        break;
                    }
                }
            }

            Some(c) => error_at(*c, loc),

            None => {
                tokens.push(Token::new(TokenKind::EOF, "".to_string(), loc));
                break;
            }
        }
    }

    tokens
}
