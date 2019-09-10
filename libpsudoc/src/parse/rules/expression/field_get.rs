use super::*;

use crate::coretypes::{Expression, Spanned, Token, TokenCategory};

pub struct FieldGet;

impl ParseFunction for FieldGet {
    type Output = Box<dyn FnOnce(Expression) -> Expression>;

    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        context.skip_whitespaces(true);
        let question = context
            .next_token_categoried(&[TokenCategory::PunctuationQuestionMark])
            .is_some();
        let dot = if let Some(dot) =
            context.next_token_categoried(&[TokenCategory::PunctuationFullStop])
        {
            dot.clone()
        } else {
            return ParseResult::Fail(false);
        };
        context.skip_whitespaces(true);
        match context
            .next_token_categoried(&[TokenCategory::IdentifierIdentifier])
            .cloned()
        {
            Some(Token {
                category: TokenCategory::IdentifierIdentifier,
                span,
            }) => {
                let text = span.source_text(&session);
                ParseResult::Success(Box::new(move |lhs| {
                    Expression::FieldGet(
                        lhs.span().joined(&span).expect("In the same file"),
                        Box::new(lhs),
                        text,
                        question,
                    )
                }))
            }
            Some(token) => {
                token
                    .span
                    .diagnostic_error("Expected identifier in field get, but no token received.")
                    .emit_to(session);
                ParseResult::Fail(true)
            }
            None => {
                dot.span
                    .diagnostic_error("Expected identifier in field get, but no token received.")
                    .emit_to(session);
                ParseResult::Fail(true)
            }
        }
    }
}
