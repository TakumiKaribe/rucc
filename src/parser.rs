use crate::node::Node;
use crate::node_kind::NodeKind;
use crate::token::Token;
use crate::token_kind::TokenKind;
use crate::variable::*;

pub(crate) struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub(crate) fn program(&mut self) -> Vec<Box<Node>> {
        let mut program: Vec<Box<Node>> = Vec::new();
        let mut locals = LVar::new();
        while self
            .tokens
            .get(self.position)
            .map_or(false, |t| !t.is_eof())
        {
            program.push(self.stmt(&mut locals));
        }
        program
    }

    fn stmt(&mut self, locals: &mut LVar) -> Box<Node> {
        let node: Box<Node>;
        if self
            .tokens
            .get(self.position)
            .map_or(false, |token| token.consume_return())
        {
            self.position += 1;
            let lhs = self.expr(locals);
            node = Box::new(Node {
                kind: NodeKind::Return,
                lhs: Some(lhs),
                rhs: None,
            });
        } else {
            node = self.expr(locals);
        };
        self.tokens
            .get(self.position)
            .unwrap_or_else(|| panic!("expected ';'"))
            .expect(";");
        self.position += 1;
        node
    }

    fn expr(&mut self, locals: &mut LVar) -> Box<Node> {
        self.assign(locals)
    }

    fn assign(&mut self, locals: &mut LVar) -> Box<Node> {
        let mut node = self.equality(locals);
        if self
            .tokens
            .get(self.position)
            .map_or(false, |t| t.consume("="))
        {
            self.position += 1;
            node = Box::new(Node {
                kind: NodeKind::Assign,
                lhs: Some(node),
                rhs: Some(self.assign(locals)),
            });
        }
        node
    }

    fn equality(&mut self, locals: &mut LVar) -> Box<Node> {
        let mut node = self.relational(locals);

        loop {
            let token = self.tokens.get(self.position).expect("token is none");

            if token.consume("==") {
                self.position += 1;
                node = Box::new(Node {
                    kind: NodeKind::Equal,
                    lhs: Some(node),
                    rhs: Some(self.relational(locals)),
                });
            } else if token.consume("!=") {
                self.position += 1;
                node = Box::new(Node {
                    kind: NodeKind::NotEqual,
                    lhs: Some(node),
                    rhs: Some(self.relational(locals)),
                });
            } else {
                break;
            }
        }

        node
    }

    fn relational(&mut self, locals: &mut LVar) -> Box<Node> {
        let mut node = self.add(locals);

        loop {
            let token = self.tokens.get(self.position).expect("token is none");

            if token.consume("<") {
                self.position += 1;
                node = Box::new(Node {
                    kind: NodeKind::LT,
                    lhs: Some(node),
                    rhs: Some(self.add(locals)),
                });
            } else if token.consume("<=") {
                self.position += 1;
                node = Box::new(Node {
                    kind: NodeKind::LTEqual,
                    lhs: Some(node),
                    rhs: Some(self.add(locals)),
                });
            } else if token.consume(">") {
                self.position += 1;
                node = Box::new(Node {
                    kind: NodeKind::GT,
                    lhs: Some(node),
                    rhs: Some(self.add(locals)),
                });
            } else if token.consume(">=") {
                self.position += 1;
                node = Box::new(Node {
                    kind: NodeKind::GTEqual,
                    lhs: Some(node),
                    rhs: Some(self.add(locals)),
                });
            } else {
                break;
            }
        }

        node
    }

    fn add(&mut self, locals: &mut LVar) -> Box<Node> {
        let mut node = self.mul(locals);

        loop {
            let token = self.tokens.get(self.position).expect("token is none");

            if token.consume("+") {
                self.position += 1;
                node = Box::new(Node {
                    kind: NodeKind::Add,
                    lhs: Some(node),
                    rhs: Some(self.mul(locals)),
                });
            } else if token.consume("-") {
                self.position += 1;
                node = Box::new(Node {
                    kind: NodeKind::Sub,
                    lhs: Some(node),
                    rhs: Some(self.mul(locals)),
                });
            } else {
                break;
            }
        }

        node
    }

    fn mul(&mut self, locals: &mut LVar) -> Box<Node> {
        let mut node = self.unary(locals);

        loop {
            let token = self.tokens.get(self.position).expect("token is none");

            if token.consume("*") {
                self.position += 1;
                node = Box::new(Node {
                    kind: NodeKind::Mul,
                    lhs: Some(node),
                    rhs: Some(self.unary(locals)),
                });
            } else if token.consume("/") {
                self.position += 1;
                node = Box::new(Node {
                    kind: NodeKind::Div,
                    lhs: Some(node),
                    rhs: Some(self.unary(locals)),
                });
            } else {
                break;
            }
        }

        node
    }

    fn unary(&mut self, locals: &mut LVar) -> Box<Node> {
        let token = self.tokens.get(self.position).expect("token is none");

        if token.consume("+") {
            self.position += 1;
            self.term(locals)
        } else if token.consume("-") {
            self.position += 1;
            Box::new(Node {
                kind: NodeKind::Sub,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Num(0),
                    lhs: None,
                    rhs: None,
                })),
                rhs: Some(self.term(locals)),
            })
        } else {
            self.term(locals)
        }
    }

    fn term(&mut self, locals: &mut LVar) -> Box<Node> {
        let token = self.tokens.get(self.position).expect("token is none");
        self.position += 1;

        if token.consume("(") {
            let node = self.expr(locals);
            self.tokens
                .get(self.position)
                .expect("tokens is none")
                .expect(")");
            self.position += 1;
            node
        } else if token.consume_ident() {
            let mut node = Node {
                kind: NodeKind::LVar(
                    (u32::from(token.raw_string.chars().nth(0).unwrap()) - u32::from('a')) * 8,
                ),
                lhs: None,
                rhs: None,
            };
            if let Some(var) = locals.find_lvar(token) {
                node.kind = NodeKind::LVar(var.offset);
            } else if let TokenKind::Ident(ref ident_name) = token.kind {
                let lvar = Var {
                    name: ident_name.clone(),
                    len: ident_name.len() as u32,
                    offset: locals.offset() + 8,
                };
                node.kind = NodeKind::LVar(lvar.offset);
                locals.push(lvar);
            } else {
                panic!("expect ident");
            };
            Box::new(node)
        } else {
            Box::new(Node {
                kind: NodeKind::Num(token.expect_number()),
                lhs: None,
                rhs: None,
            })
        }
    }
}
