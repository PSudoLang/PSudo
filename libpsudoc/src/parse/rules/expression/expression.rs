use super::*;
use crate::coretypes::Expression as ExpressionNode;

pub struct Expression;

impl ParseFunction for Expression {
    type Output = ExpressionNode;
    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        try_all(
            vec![
                BooleanLiteral::try_parse,
                StringLiteral::try_parse,
                NumberLiteral::try_parse,
            ],
            context,
            session,
        )
    }
}
