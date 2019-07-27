use super::{Declaration, Expression, Statement};
use crate::coretypes::{Span, Spanned};

pub enum Node {
    Root(Vec<Node>),
    CompilerDirective { name: String },
    Comment(Span, String),
    Declaration(Declaration),
    Statement(Statement),
}

impl Spanned for Node {
    fn span(&self) -> Span {
        match self {
            Node::Root(vec) => vec.span(),
            // TODO
            Node::CompilerDirective { .. } => Span::FIRST_COLUMN,
            Node::Comment(span, ..) => span.clone(),
            Node::Declaration(declaration) => declaration.span(),
            Node::Statement(statement) => statement.span(),
        }
    }
}

pub struct Type {
    name: String,
}
