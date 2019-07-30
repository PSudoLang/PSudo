use crate::coretypes::{Node, Token};

pub struct ParseContext {
    tokens: Vec<Token>,
    current: usize,
}

impl ParseContext {
    pub fn new(tokens: Vec<Token>) -> ParseContext {
        ParseContext { tokens, current: 0 }
    }

    pub fn has_next(&self) -> bool {
        self.tokens.len() > self.current
    }

    pub fn peek(&self) -> Option<&Token> {
        if self.has_next() {
            Some(&self.tokens[self.current])
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Option<&Token> {
        if self.has_next() {
            let result = &self.tokens[self.current];
            self.current += 1;
            Some(result)
        } else {
            None
        }
    }

    pub fn create_sandbox(&self) -> ParseContext {
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
    Skip,
    Fail,
}

impl ParseResult {
    pub fn or<F>(self, or_function: F) -> ParseResult
    where
        F: FnOnce() -> ParseResult,
    {
        match &self {
            ParseResult::Success(..) => self,
            _ => or_function(),
        }
    }
}

pub trait ParseFunction {
    fn try_parse(context: &mut ParseContext) -> ParseResult;
}

mod comment;
mod ignore;
mod root;

pub use comment::*;
pub use ignore::*;
pub use root::*;
