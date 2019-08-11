use super::node::Node;
use super::node_kind::NodeKind;
use super::token::Token;

pub(crate) fn program(
    tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>,
) -> Vec<Box<Node>> {
    let mut program: Vec<Box<Node>> = Vec::new();
    loop {
        if tokens.peek().map_or(false, |t| t.is_eof()) {
            break;
        } else {
            program.push(stmt(tokens));
        }
    }
    program
}

fn stmt(tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>) -> Box<Node> {
    let node = expr(tokens);
    tokens
        .next()
        .unwrap_or_else(|| panic!("expected ';'"))
        .expect(";");
    node
}

fn expr(tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>) -> Box<Node> {
    assign(tokens)
}

fn assign(tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>) -> Box<Node> {
    let mut node = equality(tokens);
    if tokens.peek().map_or(false, |t| t.consume("=")) {
        tokens.next();
        node = Box::new(Node {
            kind: NodeKind::Assign,
            lhs: Some(node),
            rhs: Some(assign(tokens)),
        });
    }
    node
}

fn equality(tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>) -> Box<Node> {
    let mut node = relational(tokens);

    loop {
        let token = tokens.peek().expect("token is none");

        if token.consume("==") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::Equal,
                lhs: Some(node),
                rhs: Some(relational(tokens)),
            });
        } else if token.consume("!=") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::NotEqual,
                lhs: Some(node),
                rhs: Some(relational(tokens)),
            });
        } else {
            break;
        }
    }

    node
}

fn relational(tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>) -> Box<Node> {
    let mut node = add(tokens);

    loop {
        let token = tokens.peek().expect("token is none");

        if token.consume("<") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::LT,
                lhs: Some(node),
                rhs: Some(add(tokens)),
            });
        } else if token.consume("<=") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::LTEqual,
                lhs: Some(node),
                rhs: Some(add(tokens)),
            });
        } else if token.consume(">") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::GT,
                lhs: Some(node),
                rhs: Some(add(tokens)),
            });
        } else if token.consume(">=") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::GTEqual,
                lhs: Some(node),
                rhs: Some(add(tokens)),
            });
        } else {
            break;
        }
    }

    node
}

fn add(tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>) -> Box<Node> {
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
    let mut node = unary(tokens);

    loop {
        let token = tokens.peek().expect("token is none");

        if token.consume("*") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::Mul,
                lhs: Some(node),
                rhs: Some(unary(tokens)),
            });
        } else if token.consume("/") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::Div,
                lhs: Some(node),
                rhs: Some(unary(tokens)),
            });
        } else {
            break;
        }
    }

    node
}

fn unary(tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>) -> Box<Node> {
    let token = tokens.peek().expect("token is none");

    if token.consume("+") {
        tokens.next();
        term(tokens)
    } else if token.consume("-") {
        tokens.next();
        Box::new(Node {
            kind: NodeKind::Sub,
            lhs: Some(Box::new(Node {
                kind: NodeKind::Num(0),
                lhs: None,
                rhs: None,
            })),
            rhs: Some(term(tokens)),
        })
    } else {
        term(tokens)
    }
}

fn term(tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>) -> Box<Node> {
    let token = tokens.next().expect("token is none");

    if token.consume("(") {
        let node = expr(tokens);
        tokens.next().expect("tokens is none").expect(")");
        node
    } else if token.consume_ident() {
        Box::new(Node {
            kind: NodeKind::LVar(
                (u32::from(token.raw_string.chars().nth(0).unwrap()) - u32::from('a')) * 8,
            ),
            lhs: None,
            rhs: None,
        })
    } else {
        Box::new(Node {
            kind: NodeKind::Num(token.expect_number()),
            lhs: None,
            rhs: None,
        })
    }
}
