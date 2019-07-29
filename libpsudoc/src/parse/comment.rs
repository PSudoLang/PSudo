use super::{ParseContext, ParseFunction, ParseResult};
use crate::coretypes::{Node, Token, TokenCategory};

pub struct LineComment;

impl ParseFunction for LineComment {
    fn try_parse(context: &mut ParseContext) -> ParseResult {
        if let Some(token) = context.next() {
            if token.category == TokenCategory::LineComment {
                return ParseResult::Success(Node::Comment(token.span.clone(), false));
            }
        }
        ParseResult::Fail
    }
}

pub struct BlockComment;

impl ParseFunction for BlockComment {
    fn try_parse(context: &mut ParseContext) -> ParseResult {
        if let Some(token) = context.next() {
            if token.category == TokenCategory::BlockComment {
                return  ParseResult::Success(Node::Comment(token.span.clone(), true));
            }
        }
        ParseResult::Fail
    }
}
