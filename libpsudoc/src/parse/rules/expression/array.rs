use super::*;
use crate::coretypes::TokenCategory;

use crate::coretypes::Expression as ExpressionNode;

pub struct Array;

impl ParseFunction for Array {
    type Output = ExpressionNode;

    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        let left_parenthesis = if let Some(token) = context
            .next_if_matched(|token| token.category == TokenCategory::PunctuationLeftSquareBracket)
        {
            token.clone()
        } else {
            return ParseResult::Fail(false);
        };

        let mut expressions = Vec::new();
        let mut expect_comma = false;
        let mut is_failed = false;
        context.skip_whitespaces(true);

        while let Some(token) = context.peek().cloned() {
            if token.category == TokenCategory::PunctuationRightSquareBracket {
                context.next();
                break;
            }
            if is_failed {
                context.next();
                continue;
            }
            if expect_comma {
                if token.category != TokenCategory::PunctuationComma {
                    token
                        .span
                        .diagnostic_error(format!(
                            "Expected , but {} received",
                            token.span.source_text(session).escape_debug()
                        ))
                        .emit_to(session);
                    return ParseResult::Fail(true);
                }
                context.next();
                expect_comma = false;
                context.skip_whitespaces(true);
                continue;
            }

            match Expression::try_parse(context, session) {
                ParseResult::Success(expression) => {
                    expressions.push(expression);
                    expect_comma = true;
                }
                ParseResult::Fail(_) => {
                    token
                        .span
                        .diagnostic_error(format!(
                            "Expected expression in array, but {} received",
                            token.span.source_text(session).escape_debug()
                        ))
                        .emit_to(session);
                    is_failed = true;
                }
            }
        }
        if is_failed {
            return ParseResult::Fail(true);
        }

        let span = left_parenthesis
            .span
            .clone()
            .joined(&context.last_read_token().span)
            .expect("In the same file");

        ParseResult::Success(ExpressionNode::Array(span, expressions))
    }
}
