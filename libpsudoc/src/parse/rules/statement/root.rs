use super::*;
use crate::coretypes::Statement as StatementNode;

pub struct Statement;

impl ParseFunction for Statement {
    type Output = StatementNode;
    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        try_all(
            &[
                Output::try_parse,
                ExpressionStatement::try_parse,
                DeclarationStatement::try_parse,
            ],
            context,
            session,
        )
    }
}
