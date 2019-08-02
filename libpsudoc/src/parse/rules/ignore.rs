use super::{ParseContext, ParseFunction, ParseResult};
use crate::coretypes::{CompileSession, Node, TokenCategory};

pub struct Whitespace;

impl ParseFunction for Whitespace {
    type Output = Node;
    fn try_parse(context: &mut ParseContext, _: &mut CompileSession) -> ParseResult<Self::Output> {
        if let Some(token) = context.next() {
            if token.category == TokenCategory::Separator
                || token.category == TokenCategory::Whitespace
            {
                return ParseResult::Skip;
            }
        }
        ParseResult::Fail(false)
    }
}
