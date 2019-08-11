use super::node::Node;
use super::node_kind::NodeKind;
use super::token::Token;

pub(crate) fn expr(tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>) -> Box<Node> {
    equality(tokens)
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
    } else {
        Box::new(Node {
            kind: NodeKind::Num(token.expect_number()),
            lhs: None,
            rhs: None,
        })
    }
}
