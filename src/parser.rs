use super::node::Node;
use super::node_kind::NodeKind;
use super::token::Token;
use super::token_kind::TokenKind;
use super::variable::*;

pub(crate) fn program(
    tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>,
) -> Vec<Box<Node>> {
    let mut program: Vec<Box<Node>> = Vec::new();
    let mut locals = LVar::new();
    while tokens.peek().map_or(false, |t| !t.is_eof()) {
        program.push(stmt(tokens, &mut locals));
    }
    program
}

fn stmt(
    tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>,
    locals: &mut LVar,
) -> Box<Node> {
    let node = expr(tokens, locals);
    tokens
        .next()
        .unwrap_or_else(|| panic!("expected ';'"))
        .expect(";");
    node
}

fn expr(
    tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>,
    locals: &mut LVar,
) -> Box<Node> {
    assign(tokens, locals)
}

fn assign(
    tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>,
    locals: &mut LVar,
) -> Box<Node> {
    let mut node = equality(tokens, locals);
    if tokens.peek().map_or(false, |t| t.consume("=")) {
        tokens.next();
        node = Box::new(Node {
            kind: NodeKind::Assign,
            lhs: Some(node),
            rhs: Some(assign(tokens, locals)),
        });
    }
    node
}

fn equality(
    tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>,
    locals: &mut LVar,
) -> Box<Node> {
    let mut node = relational(tokens, locals);

    loop {
        let token = tokens.peek().expect("token is none");

        if token.consume("==") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::Equal,
                lhs: Some(node),
                rhs: Some(relational(tokens, locals)),
            });
        } else if token.consume("!=") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::NotEqual,
                lhs: Some(node),
                rhs: Some(relational(tokens, locals)),
            });
        } else {
            break;
        }
    }

    node
}

fn relational(
    tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>,
    locals: &mut LVar,
) -> Box<Node> {
    let mut node = add(tokens, locals);

    loop {
        let token = tokens.peek().expect("token is none");

        if token.consume("<") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::LT,
                lhs: Some(node),
                rhs: Some(add(tokens, locals)),
            });
        } else if token.consume("<=") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::LTEqual,
                lhs: Some(node),
                rhs: Some(add(tokens, locals)),
            });
        } else if token.consume(">") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::GT,
                lhs: Some(node),
                rhs: Some(add(tokens, locals)),
            });
        } else if token.consume(">=") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::GTEqual,
                lhs: Some(node),
                rhs: Some(add(tokens, locals)),
            });
        } else {
            break;
        }
    }

    node
}

fn add(
    tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>,
    locals: &mut LVar,
) -> Box<Node> {
    let mut node = mul(tokens, locals);

    loop {
        let token = tokens.peek().expect("token is none");

        if token.consume("+") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::Add,
                lhs: Some(node),
                rhs: Some(mul(tokens, locals)),
            });
        } else if token.consume("-") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::Sub,
                lhs: Some(node),
                rhs: Some(mul(tokens, locals)),
            });
        } else {
            break;
        }
    }

    node
}

fn mul(
    tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>,
    locals: &mut LVar,
) -> Box<Node> {
    let mut node = unary(tokens, locals);

    loop {
        let token = tokens.peek().expect("token is none");

        if token.consume("*") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::Mul,
                lhs: Some(node),
                rhs: Some(unary(tokens, locals)),
            });
        } else if token.consume("/") {
            tokens.next();
            node = Box::new(Node {
                kind: NodeKind::Div,
                lhs: Some(node),
                rhs: Some(unary(tokens, locals)),
            });
        } else {
            break;
        }
    }

    node
}

fn unary(
    tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>,
    locals: &mut LVar,
) -> Box<Node> {
    let token = tokens.peek().expect("token is none");

    if token.consume("+") {
        tokens.next();
        term(tokens, locals)
    } else if token.consume("-") {
        tokens.next();
        Box::new(Node {
            kind: NodeKind::Sub,
            lhs: Some(Box::new(Node {
                kind: NodeKind::Num(0),
                lhs: None,
                rhs: None,
            })),
            rhs: Some(term(tokens, locals)),
        })
    } else {
        term(tokens, locals)
    }
}

fn term(
    tokens: &mut core::iter::Peekable<std::slice::Iter<'_, Token>>,
    locals: &mut LVar,
) -> Box<Node> {
    let token = tokens.next().expect("token is none");

    if token.consume("(") {
        let node = expr(tokens, locals);
        tokens.next().expect("tokens is none").expect(")");
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
