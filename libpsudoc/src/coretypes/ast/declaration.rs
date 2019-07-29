use super::{Expression, Type};
use crate::coretypes::{Span, Spanned};

pub enum Declaration {
    Variable(Span, String, Expression),
    Struct(Span, Vec<(String, Type)>),
    Enum(Span),
    // TODO
    Function(Span, String),
    Private(Box<Declaration>),
}

impl Spanned for Declaration {
    fn span(&self) -> Span {
        match self {
            Declaration::Variable(span, ..) => span.clone(),
            Declaration::Struct(span, ..) => span.clone(),
            Declaration::Enum(span, ..) => span.clone(),
            Declaration::Function(span, ..) => span.clone(),
            Declaration::Private(declaration) => declaration.span(),
        }
    }
}
