use super::{ParseContext, ParseFunction, ParseResult};
use crate::coretypes::TokenCategory;

pub struct Whitespace;

impl ParseFunction for Whitespace {
    fn try_parse(context: &mut ParseContext) -> ParseResult {
        if let Some(token) = context.next() {
            if token.category == TokenCategory::Separator
                || token.category == TokenCategory::Whitespace
            {
                return ParseResult::Skip;
            }
        }
        ParseResult::Fail
    }
}
