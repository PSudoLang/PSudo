use super::Statement;
use crate::coretypes::{Span, Spanned};

pub enum TypeName {
    Unit,
    Never,
    Path(String),
}

impl ToString for TypeName {
    fn to_string(&self) -> String {
        match self {
            TypeName::Unit => "()".into(),
            TypeName::Never => "!".into(),
            TypeName::Path(s) => s.clone(),
        }
    }
}

pub struct Type {
    pub span: Span,
    pub name: TypeName,
}

impl Spanned for Type {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

pub type TypedIdentifier = (String, Type);

pub struct Block(pub Span, pub Vec<Statement>);

impl Spanned for Block {
    fn span(&self) -> Span {
        self.0.clone()
    }
}
