use super::*;

use crate::coretypes::Statement;

pub struct DeclarationStatement;

impl ParseFunction for DeclarationStatement {
    type Output = Statement;
    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        Declaration::try_parse(context, session).map(Statement::Declaration)
    }
}
