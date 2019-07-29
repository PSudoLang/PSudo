use super::{Declaration, Expression};
use crate::coretypes::{Span, Spanned};

pub enum Statement {
    Expression(Span, Expression),
    Return(Span, Option<Expression>),
    Assign(Span, String, Expression),
    Input(Span),
    Output(Span),
    Declaration(Declaration),
}

impl Spanned for Statement {
    fn span(&self) -> Span {
        match self {
            Statement::Expression(span, ..) => span.clone(),
            Statement::Return(span, ..) => span.clone(),
            Statement::Assign(span, ..) => span.clone(),
            Statement::Input(span, ..) => span.clone(),
            Statement::Output(span, ..) => span.clone(),
            Statement::Declaration(declaration) => declaration.span(),
        }
    }
}
