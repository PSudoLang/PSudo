use super::*;

use crate::coretypes::Statement;

pub struct ExpressionStatement;

impl ParseFunction for ExpressionStatement {
    type Output = Statement;
    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        Expression::try_parse(context, session).map(Statement::Expression)
    }
}
