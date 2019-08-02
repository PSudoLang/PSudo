use super::{ParseContext, ParseFunction, ParseResult};
use crate::coretypes::{CompileSession, Node, TokenCategory};

pub struct LineComment;

impl ParseFunction for LineComment {
    type Output = Node;
    fn try_parse(context: &mut ParseContext, _: &mut CompileSession) -> ParseResult<Self::Output> {
        if let Some(token) = context.next() {
            if token.category == TokenCategory::LineComment {
                return ParseResult::Success(Node::Comment(token.span.clone(), false));
            }
        }
        ParseResult::Fail(false)
    }
}

pub struct BlockComment;

impl ParseFunction for BlockComment {
    type Output = Node;
    fn try_parse(context: &mut ParseContext, _: &mut CompileSession) -> ParseResult<Self::Output> {
        if let Some(token) = context.next() {
            if token.category == TokenCategory::BlockComment {
                return ParseResult::Success(Node::Comment(token.span.clone(), true));
            }
        }
        ParseResult::Fail(false)
    }
}
