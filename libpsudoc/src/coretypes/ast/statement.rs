use crate::coretypes::{Block, Declaration, Expression, Span, Spanned};

type ToPrintLinefeed = bool;

pub enum Statement {
    Expression(Expression),
    Return(Span, Option<Expression>),
    Assign(Span, String, Expression),
    Input(Span),
    Output(Span, Vec<Expression>, ToPrintLinefeed),
    Declaration(Declaration),
    Block(Block),
}

impl Spanned for Statement {
    fn span(&self) -> Span {
        match self {
            Statement::Expression(expression) => expression.span(),
            Statement::Return(span, ..) => span.clone(),
            Statement::Assign(span, ..) => span.clone(),
            Statement::Input(span, ..) => span.clone(),
            Statement::Output(span, ..) => span.clone(),
            Statement::Declaration(declaration) => declaration.span(),
            Statement::Block(block) => block.span(),
        }
    }
}
