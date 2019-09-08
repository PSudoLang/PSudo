use super::*;
use crate::coretypes::{Span, TokenCategory, Type as TypeData};

pub struct TypeAnnotation;

impl ParseFunction for TypeAnnotation {
    type Output = (Span, TypeData);

    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        let colon = if let Some(token) =
            context.next_if_matched(|token| token.category == TokenCategory::PunctuationColon)
        {
            token.clone()
        } else {
            return ParseResult::Fail(false);
        };

        context.skip_whitespaces(true);

        match Type::try_parse(context, session) {
            ParseResult::Success(r#type) => ParseResult::Success((
                colon.span.joined(&r#type.span).expect("In the same file"),
                r#type,
            )),
            ParseResult::Fail(_) => ParseResult::Fail(true),
        }
    }
}
