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

    pub(crate) fn program(&mut self) -> Vec<Node> {
        let mut program: Vec<Node> = vec![];
        let mut locals: Option<Box<Var>> = None;
        while self
            .tokens
            .get(self.position)
            .map_or(false, |t| !t.is_eof())
        {
            program.push(self.stmt(&mut locals));
        }
        program
    }

    fn stmt(&mut self, locals: &mut Option<Box<Var>>) -> Node {
        let node: Node;
        if self
            .tokens
            .get(self.position)
            .map_or(false, |token| token.consume_return())
        {
            self.position += 1;
            let lhs = self.expr(locals);
            node = Node {
                kind: NodeKind::Return,
                lhs: Some(Box::new(lhs)),
                rhs: None,
            };
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

    fn expr(&mut self, locals: &mut Option<Box<Var>>) -> Node {
        self.assign(locals)
    }

    fn assign(&mut self, locals: &mut Option<Box<Var>>) -> Node {
        let mut node = self.equality(locals);
        if self
            .tokens
            .get(self.position)
            .map_or(false, |t| t.consume("="))
        {
            self.position += 1;
            node = Node {
                kind: NodeKind::Assign,
                lhs: Some(Box::new(node)),
                rhs: Some(Box::new(self.assign(locals))),
            };
        }
        node
    }

    fn equality(&mut self, locals: &mut Option<Box<Var>>) -> Node {
        let mut node = self.relational(locals);

        loop {
            let token = self.tokens.get(self.position).expect("token is none");

            if token.consume("==") {
                self.position += 1;
                node = Node {
                    kind: NodeKind::Equal,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.relational(locals))),
                };
            } else if token.consume("!=") {
                self.position += 1;
                node = Node {
                    kind: NodeKind::NotEqual,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.relational(locals))),
                };
            } else {
                break;
            }
        }

        node
    }

    fn relational(&mut self, locals: &mut Option<Box<Var>>) -> Node {
        let mut node = self.add(locals);

        loop {
            let token = self.tokens.get(self.position).expect("token is none");

            if token.consume("<") {
                self.position += 1;
                node = Node {
                    kind: NodeKind::LT,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.add(locals))),
                };
            } else if token.consume("<=") {
                self.position += 1;
                node = Node {
                    kind: NodeKind::LTEqual,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.add(locals))),
                };
            } else if token.consume(">") {
                self.position += 1;
                node = Node {
                    kind: NodeKind::GT,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.add(locals))),
                };
            } else if token.consume(">=") {
                self.position += 1;
                node = Node {
                    kind: NodeKind::GTEqual,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.add(locals))),
                };
            } else {
                break;
            }
        }

        node
    }

    fn add(&mut self, locals: &mut Option<Box<Var>>) -> Node {
        let mut node = self.mul(locals);

        loop {
            let token = self.tokens.get(self.position).expect("token is none");

            if token.consume("+") {
                self.position += 1;
                node = Node {
                    kind: NodeKind::Add,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.mul(locals))),
                };
            } else if token.consume("-") {
                self.position += 1;
                node = Node {
                    kind: NodeKind::Sub,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.mul(locals))),
                };
            } else {
                break;
            }
        }

        node
    }

    fn mul(&mut self, locals: &mut Option<Box<Var>>) -> Node {
        let mut node = self.unary(locals);

        loop {
            let token = self.tokens.get(self.position).expect("token is none");

            if token.consume("*") {
                self.position += 1;
                node = Node {
                    kind: NodeKind::Mul,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.unary(locals))),
                };
            } else if token.consume("/") {
                self.position += 1;
                node = Node {
                    kind: NodeKind::Div,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(self.unary(locals))),
                };
            } else {
                break;
            }
        }

        node
    }

    fn unary(&mut self, locals: &mut Option<Box<Var>>) -> Node {
        let token = self.tokens.get(self.position).expect("token is none");

        if token.consume("+") {
            self.position += 1;
            self.unary(locals)
        } else if token.consume("-") {
            self.position += 1;
            Node {
                kind: NodeKind::Sub,
                lhs: Some(Box::new(Node {
                    kind: NodeKind::Num(0),
                    lhs: None,
                    rhs: None,
                })),
                rhs: Some(Box::new(self.unary(locals))),
            }
        } else {
            self.primary(locals)
        }
    }

    fn primary(&mut self, locals: &mut Option<Box<Var>>) -> Node {
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
            if let Some(var) = locals.as_ref().and_then(|var| var.find_lvar(token)) {
                Node {
                    kind: NodeKind::LVar(var.offset),
                    lhs: None,
                    rhs: None,
                }
            } else if let TokenKind::Ident(ref ident_name) = token.kind {
                let local = locals.take();
                let mut var = Var {
                    next: None,
                    name: ident_name.clone(),
                    offset: 0,
                };

                var.offset = local.as_ref().expect("local variable is none").offset + 8;
                var.next = local;
                let node = Node {
                    kind: NodeKind::LVar(var.offset),
                    lhs: None,
                    rhs: None,
                };
                std::mem::replace(locals, Some(Box::new(var)));
                node
            } else {
                panic!("expect ident");
            }
        } else {
            Node {
                kind: NodeKind::Num(token.expect_number()),
                lhs: None,
                rhs: None,
            }
        }
    }
}
