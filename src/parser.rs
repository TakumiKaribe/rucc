use super::node::Node;
use super::node_kind::NodeKind;
use super::token::Token;

pub(crate) fn expr(tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>) -> Box<Node> {
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
    let mut node = term(tokens);

    loop {
        let token = tokens.peek().expect("token is none");

        if token.consume("*") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::Mul,
                lhs: Some(node),
                rhs: Some(term(tokens)),
            });
        } else if token.consume("/") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::Div,
                lhs: Some(node),
                rhs: Some(term(tokens)),
            });
        } else {
            break;
        }
    }

    node
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
