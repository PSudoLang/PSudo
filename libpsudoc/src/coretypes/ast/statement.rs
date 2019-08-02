use super::{Declaration, Expression};
use crate::coretypes::{indented, CompileSession, RichDebug, Span, Spanned};

type ToPrintLinefeed = bool;

pub enum Statement {
    Expression(Expression),
    Return(Span, Option<Expression>),
    Assign(Span, String, Expression),
    Input(Span),
    Output(Span, Vec<Expression>, ToPrintLinefeed),
    Declaration(Declaration),
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
        }
    }
}

impl RichDebug for Statement {
    fn rich_debug(&self, session: &CompileSession) -> String {
        match self {
            Statement::Output(_, expressions, to_print_linefeed) => format!(
                "Output{} {{\n{}\n}}",
                if *to_print_linefeed {
                    ""
                } else {
                    "(No Linefeed)"
                },
                indented(
                    expressions
                        .iter()
                        .map(|expression| expression.rich_debug(session))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            ),
            _ => "Unknown Statement".into(),
        }
    }
}
