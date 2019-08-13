use super::*;
use crate::util::SemiDebug;
use crate::coretypes::{TokenCategory, Block as BlockNode};

pub struct Block;

impl ParseFunction for Block {
    type Output = BlockNode;

    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        let left_parenthesis = if let Some(token) = context.next_if_matched(|token| {
            token.category == TokenCategory::GroupOpen && token.span.source_text(session) == "{"
        }) {
            token.clone()
        } else {
            return ParseResult::Fail(false);
        };

        let mut statements = Vec::new();
        let mut is_failed = false;
        context.skip_whitespaces(true);

        while let Some(token) = context.peek().cloned() {
            if token.category == TokenCategory::GroupClose && token.span.source_text(session) == "}"
            {
                context.next();
                break;
            }
            if token.category.can_be_separator() {
                context.next();
                continue;
            }
            if is_failed {
                context.next();
                continue;
            }

            match Statement::try_parse(context, session) {
                ParseResult::Success(statement) => {
                    statements.push(statement);
                }
                ParseResult::Fail(_) => {
                    token
                        .span
                        .diagnostic_error(format!(
                            "Expected expression but {} received",
                            token.span.source_text(session).semi_debug()
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

        ParseResult::Success(BlockNode(span, statements))
    }
}