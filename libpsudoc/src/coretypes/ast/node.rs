use super::Statement;
use crate::coretypes::{Span, Spanned};

type IsBlockComment = bool;

pub enum Node {
    Root(Vec<Node>),
    CompilerDirective { name: String },
    Comment(Span, IsBlockComment),
    Statement(Statement),
}

impl Spanned for Node {
    fn span(&self) -> Span {
        match self {
            Node::Root(vec) => vec.span(),
            // TODO
            Node::CompilerDirective { .. } => Span::FIRST_COLUMN,
            Node::Comment(span, ..) => span.clone(),
            Node::Statement(statement) => statement.span(),
        }
    }
}

pub struct Type {
    name: String,
}
