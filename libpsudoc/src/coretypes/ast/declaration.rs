use crate::coretypes::{Block, Span, Spanned, Type, TypedIdentifier, Expression};

pub enum Declaration {
    Variable(Span, String, Expression),
    Struct(Span, Vec<(String, Type)>),
    Enum(Span),
    // TODO
    Function(Span, String, Vec<TypedIdentifier>, Type, Block),
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
