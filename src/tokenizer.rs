use crate::location::Location;
use crate::token::Token;
use crate::token_kind::*;

#[derive(Default)]
pub(crate) struct Tokenizer {
    input: Vec<char>,
    position: usize,
    examining_char: Option<char>,
}

impl Tokenizer {
    pub(crate) fn new(input: String) -> Self {
        let mut tokenizer = Self {
            input: input.chars().collect(),
            ..Default::default()
        };
        tokenizer.examining_char = tokenizer.input.get(tokenizer.position).cloned();
        tokenizer
    }

    pub(crate) fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        let mut loc = Location { at: 0, len: 0 };

        loop {
            match self.examining_char {
                Some(ch) if ch.is_whitespace() => {
                    self.position += 1;
                    loc.succ(1);
                }

                Some(op)
                    if op == '+'
                        || op == '-'
                        || op == '*'
                        || op == '/'
                        || op == '('
                        || op == ')'
                        || op == ';' =>
                {
                    loc.len(1);
                    tokens.push(Token::new(
                        TokenKind::Reserved(op.to_string()),
                        op.to_string(),
                        loc,
                    ));
                    self.position += 1;
                    loc.succ(1);
                }

                Some(op) if op == '=' || op == '<' || op == '>' => {
                    let mut op = self.examining_char.unwrap().to_string();
                    loc.len(1);
                    if let Some(&'=') = self.input.get(self.position + 1) {
                        self.position += 1;
                        op.push(self.input.get(self.position).cloned().unwrap_or_else(|| {
                            panic!("self.position '{}' is out of bounds", self.position)
                        }));
                        loc.len(2);
                    }
                    tokens.push(Token::new(TokenKind::Reserved(op.clone()), op, loc));
                    if loc.len == 2 {
                        loc.succ(2);
                    } else {
                        loc.succ(1);
                    }
                }

                Some(op) if op == '!' => {
                    if let Some(&'=') = self.input.get(self.position + 1) {
                        loc.len(2);
                        tokens.push(Token::new(
                            TokenKind::Reserved("!=".to_string()),
                            "!=".to_string(),
                            loc,
                        ));
                        loc.succ(2);
                        self.position += 1;
                    } else {
                        error_at(
                            self.examining_char
                                .unwrap_or_else(|| panic!("peeking char is None. expect '='")),
                            loc,
                        );
                    }
                }

                Some(ref c) if c.is_ascii_alphabetic() || c == &'_' => {
                    let mut var = self.examining_char.unwrap().to_string();
                    let mut len = 1;
                    self.position += 1;
                    while self
                        .input
                        .get(self.position)
                        .map_or(false, |c| c.is_ascii_alphanumeric() || c == &'_')
                    {
                        var.push(self.input.get(self.position).cloned().unwrap());
                        len += 1;
                        self.position += 1;
                    }
                    loc.len(len);
                    match KEYWORD.get(var.as_str()) {
                        Some(kind) => tokens.push(Token::new(kind.clone(), var, loc)),
                        None => tokens.push(Token::new(TokenKind::Ident(var.clone()), var, loc)),
                    }
                    loc.succ(len);
                }

                Some(n) if n.is_ascii_digit() => {
                    let mut num = 0;
                    let mut n = self.examining_char.unwrap();
                    loop {
                        num *= 10;
                        num += n.to_digit(10).unwrap();
                        self.position += 1;
                        if self
                            .input
                            .get(self.position)
                            .map_or(false, char::is_ascii_digit)
                        {
                            n = self.input.get(self.position).cloned().unwrap();
                        } else {
                            let digit = f64::from(num).log10() as u32 + 1;
                            loc.len(digit);
                            tokens.push(Token::new(TokenKind::Num(num), num.to_string(), loc));
                            loc.succ(digit);
                            break;
                        }
                    }
                }

                Some(c) => error_at(c, loc),

                None => {
                    tokens.push(Token::new(TokenKind::EOF, "".to_string(), loc));
                    break;
                }
            }
            self.examining_char = self.input.get(self.position).cloned();
        }

        tokens
    }
}

fn error_at(c: char, loc: Location) {
    (0..loc.at).for_each(|_| print!(" "));
    print!("^");
    println!(" invalid charactor is '{}'", c);
    panic!();
}
