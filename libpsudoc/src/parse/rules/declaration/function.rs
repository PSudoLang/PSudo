use super::*;
use crate::coretypes::{TokenCategory, Type, TypeName};

use crate::coretypes::Declaration;

pub struct Function;

impl ParseFunction for Function {
    type Output = Declaration;

    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        let fn_keyword = if let Some(token) = context.next_if_matched(|token| {
            token.category == TokenCategory::Keyword && token.span.source_text(session) == "fn"
        }) {
            token.clone()
        } else {
            return ParseResult::Fail(false);
        };

        context.skip_whitespaces(false);

        let function_name = if let Some(token) = context.next() {
            if token.category == TokenCategory::IdentifierIdentifier {
                token.span.source_text(session)
            } else {
                token
                    .span
                    .diagnostic_error(format!(
                        "Expected identifier after fn keyword but {} recevied.",
                        token.span.source_text(session).escape_debug()
                    ))
                    .emit_to(session);
                return ParseResult::Fail(true);
            }
        } else {
            context
                .last_read_token()
                .span
                .diagnostic_error("Expected identifier after fn keyword but no token recevied.")
                .emit_to(session);
            return ParseResult::Fail(true);
        };
        context.skip_whitespaces(false);

        if let Some(token) = context.next() {
            if token.category != TokenCategory::PunctuationLeftParenthesis {
                token
                    .span
                    .diagnostic_error(format!(
                        "Expected ( after identifier but {} recevied.",
                        token.span.source_text(session).escape_debug()
                    ))
                    .emit_to(session);
                return ParseResult::Fail(true);
            }
        } else {
            context
                .last_read_token()
                .span
                .diagnostic_error("Expected ( after identifier but no token recevied.")
                .emit_to(session);
            return ParseResult::Fail(true);
        };

        let mut type_variables = Vec::new();
        let mut expect_comma = false;
        let mut is_failed = false;
        context.skip_whitespaces(true);

        while let Some(token) = context.peek().cloned() {
            if token.category == TokenCategory::PunctuationRightParenthesis {
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

            let variable_name = if let Some(token) = context.next() {
                if token.category == TokenCategory::IdentifierIdentifier {
                    token.clone()
                } else {
                    token
                        .span
                        .diagnostic_error(format!(
                            "Expected identifier but {} recevied.",
                            token.span.source_text(session).escape_debug()
                        ))
                        .emit_to(session);
                    is_failed = true;
                    continue;
                }
            } else {
                context
                    .last_read_token()
                    .span
                    .diagnostic_error("Expected identifier but no token recevied.")
                    .emit_to(session);
                is_failed = true;
                continue;
            };
            context.skip_whitespaces(true);

            match TypeAnnotation::try_parse(context, session) {
                ParseResult::Success(type_annotation) => {
                    type_variables
                        .push((variable_name.span.source_text(session), type_annotation.1));
                    expect_comma = true;
                }
                ParseResult::Fail(_) => {
                    token
                        .span
                        .diagnostic_error(format!(
                            "Expected type annotation but {} received",
                            token.span.source_text(session).escape_debug()
                        ))
                        .emit_to(session);
                    is_failed = true;
                }
            }
        }

        context.skip_whitespaces(true);

        let return_type = if let Some(token) = context.peek().cloned() {
            if token.category == TokenCategory::PunctuationColon {
                match TypeAnnotation::try_parse(context, session) {
                    ParseResult::Success(type_annotation) => Some(type_annotation.1),
                    ParseResult::Fail(_) => {
                        token
                            .span
                            .diagnostic_error(format!(
                                "Expected type annotation but {} received",
                                token.span.source_text(session).escape_debug()
                            ))
                            .emit_to(session);
                        is_failed = true;
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
        .unwrap_or_else(|| Type {
            span: context.last_read_token().span.clone(),
            name: TypeName::Unit,
        });

        context.skip_whitespaces(true);
        let block_body = if let Some(token) = context.peek().cloned() {
            if token.category == TokenCategory::PunctuationLeftCurlyBracket {
                match Block::try_parse(context, session) {
                    ParseResult::Success(block) => block,
                    ParseResult::Fail(val) => {
                        if !val {
                            token
                                .span
                                .diagnostic_error(format!(
                                    "Expected block body but {} received",
                                    token.span.source_text(session).escape_debug()
                                ))
                                .emit_to(session);
                        }
                        return ParseResult::Fail(true);
                    }
                }
            } else {
                token
                    .span
                    .diagnostic_error(format!(
                        "Expected block body but {} received",
                        token.span.source_text(session).escape_debug()
                    ))
                    .emit_to(session);
                return ParseResult::Fail(true);
            }
        } else {
            return ParseResult::Fail(true);
        };

        if is_failed {
            return ParseResult::Fail(true);
        }

        ParseResult::Success(Declaration::Function(
            fn_keyword
                .span
                .joined(&context.last_read_token().span)
                .expect("In the same file"),
            function_name,
            type_variables,
            return_type,
            block_body,
        ))
    }
}
