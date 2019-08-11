use super::token::Token;
use super::token_kind::*;

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
                    TokenKind::Reserved(op.to_string(), loc),
                    op.to_string(),
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
                tokens.push(Token::new(TokenKind::Reserved(op.clone(), loc), op));
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
                        TokenKind::Reserved("!=".to_string(), loc),
                        "!=".to_string(),
                    ));
                    loc.succ(2);
                    program.next();
                    program.next();
                } else {
                    error_at(*program.peek().unwrap(), loc);
                }
            }

            Some(var) if ('a'..='z').contains(var) => {
                let mut var = program.next().unwrap().to_string();
                let mut len = 1;
                while program.peek().map_or(false, |c| ('a'..='z').contains(c)) {
                    var.push(program.next().unwrap());
                    len += 1;
                }
                loc.len(len);
                tokens.push(Token::new(TokenKind::Ident(var.clone(), loc), var));
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
                        tokens.push(Token::new(TokenKind::Num(num, loc), num.to_string()));
                        loc.succ(digit);
                        num = 0;
                        break;
                    }
                }
            }

            Some(c) => error_at(*c, loc),

            None => {
                tokens.push(Token::new(TokenKind::EOF, "".to_string()));
                break;
            }
        }
    }

    tokens
}
