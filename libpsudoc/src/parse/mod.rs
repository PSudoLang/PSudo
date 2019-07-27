use crate::coretypes::{Node, Token};

pub struct ParseContext {
    tokens: Vec<Token>,
    current: usize,
}

impl ParseContext {
    pub fn new(tokens: Vec<Token>) -> ParseContext {
        ParseContext { tokens, current: 0 }
    }

    fn create_sandbox(&self) -> ParseContext {
        ParseContext {
            tokens: self
                .tokens
                .clone()
                .into_iter()
                .skip(self.current)
                .collect::<Vec<_>>(),
            current: 0,
        }
    }
}

pub enum ParseResult {
    Success(Node),
    Fail,
}

pub trait ParseFunction {
    fn can_accept(token: Token) -> bool;
    fn try_parse(context: ParseContext) -> ParseResult;
}
