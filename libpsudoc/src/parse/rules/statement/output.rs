use super::*;
use crate::coretypes::{Statement, TokenCategory};
use crate::util::SemiDebug;

pub struct Output;

impl ParseFunction for Output {
    type Output = Statement;
    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        let output_keyword = if let Some(token) = context.next().cloned() {
            token
        } else {
            return ParseResult::Fail(false);
        };

        if output_keyword.category != TokenCategory::Keyword
            || output_keyword.span.source_text(session) != "output"
        {
            return ParseResult::Fail(false);
        }

        context.skip_whitespaces(false);

        let mut expressions = Vec::new();
        let mut to_print_linefeed = true;
        let mut is_failed = false;
        let mut expect_comma = false;

        while let Some(token) = context.peek().cloned() {
            if token.category.can_be_separator() {
                context.next();
                break;
            }
            if is_failed {
                context.next();
                continue;
            }
            if !to_print_linefeed {
                token
                    .span
                    .diagnostic_error(format!(
                        "Expected separator but {} received.",
                        token.span.source_text(session).semi_debug()
                    ))
                    .emit_to(session);
                return ParseResult::Fail(true);
            }
            if token.span.source_text(session) == "\\" {
                to_print_linefeed = false;
                context.next();
                continue;
            }
            if expect_comma {
                if token.category != TokenCategory::Punctuation
                    || token.span.source_text(session) != ","
                {
                    token
                        .span
                        .diagnostic_error(format!(
                            "Expected comma but {} received",
                            token.span.source_text(session).semi_debug()
                        ))
                        .emit_to(session);
                    is_failed = true;
                }
                context.next();
                expect_comma = false;
                context.skip_whitespaces(false);
                continue;
            }
            let result = try_all(vec![Expression::try_parse], context, session);
            match result {
                ParseResult::Success(expression) => {
                    expressions.push(expression);
                    expect_comma = true;
                }
                ParseResult::Fail(value) => {
                    if !value {
                        token
                            .span
                            .diagnostic_error(format!(
                                "Expected expression but {} received",
                                token.span.source_text(session).semi_debug()
                            ))
                            .emit_to(session);
                    }
                    is_failed = true;
                }
            }
            context.skip_whitespaces(false);
        }

        if is_failed {
            return ParseResult::Fail(true);
        }

        ParseResult::Success(Statement::Output(
            output_keyword
                .span
                .clone()
                .joined(context.last_read_token().span.clone())
                .expect("In the same file"),
            expressions,
            to_print_linefeed,
        ))
    }
}
