use super::*;
use crate::coretypes::{Expression, Literal, TokenCategory};

pub struct BooleanLiteral;

impl ParseFunction for BooleanLiteral {
    type Output = Expression;

    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        if let Some(token) = context.next() {
            if token.category == TokenCategory::LiteralBoolean {
                return ParseResult::Success(Expression::Literal(Literal::Boolean(
                    token.span.clone(),
                    token.span.source_text(session) == "true",
                )));
            }
        }
        ParseResult::Fail(false)
    }
}

pub struct StringLiteral;

impl ParseFunction for StringLiteral {
    type Output = Expression;

    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        if let Some(token) = context.next() {
            if token.category == TokenCategory::LiteralString {
                // TODO: validation of escape sequence and char.
                return ParseResult::Success(Expression::Literal(Literal::String(
                    token.span.clone(),
                    {
                        let text = token.span.source_text(session);
                        text.chars()
                            .skip(1)
                            .take(text.chars().count() - 2)
                            .collect::<String>()
                    },
                )));
            }
        }
        ParseResult::Fail(false)
    }
}

pub struct NumberLiteral;

impl ParseFunction for NumberLiteral {
    type Output = Expression;

    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        if let Some(token) = context.next().cloned() {
            if token.category == TokenCategory::LiteralNumber {
                return ParseResult::Success(Expression::Literal(
                    if token.span.source_text(session).contains('.') {
                        Literal::Decimal(token.span)
                    } else {
                        Literal::Integer(token.span)
                    },
                ));
            }
        }
        ParseResult::Fail(false)
    }
}
