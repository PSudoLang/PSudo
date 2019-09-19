use super::*;
use crate::coretypes::{TokenCategory, Expression as ExprssionNode, ControlFlowExpression};

pub struct Return;

impl ParseFunction for Return {
    type Output = ExprssionNode;

    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        let return_keyword = if let Some(token) = context
            .next_token_categoried(&[TokenCategory::KeywordReturn])
        {
            token.clone()
        } else {
            return ParseResult::Fail(false);
        };

        context.skip_whitespaces(true);

        let expression = match Expression::try_parse(context, session) {
                ParseResult::Success(expression) => expression,
                ParseResult::Fail(_) => {
                    return_keyword
                        .span
                        .diagnostic_error(format!(
                            "Expected expression in return, but {} received",
                            return_keyword.span.source_text(session).escape_debug()
                        ))
                        .emit_to(session);
                    return ParseResult::Fail(true);
                
            }
        };

        let span = return_keyword
            .span
            .clone()
            .joined(&context.last_read_token().span)
            .expect("In the same file");

        ParseResult::Success(ExprssionNode::ControlFlow(ControlFlowExpression::Return(span, Box::new(expression))))
    }
}
