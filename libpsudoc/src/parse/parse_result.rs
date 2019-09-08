type TokenUsed = bool;

pub enum ParseResult<T> {
    Success(T),
    Fail(TokenUsed),
}

impl<T> ParseResult<T> {
    pub fn map<F, R>(self, mapper: F) -> ParseResult<R>
    where
        F: FnOnce(T) -> R,
    {
        match self {
            ParseResult::Success(value) => ParseResult::Success(mapper(value)),
            ParseResult::Fail(val) => ParseResult::Fail(val),
        }
    }
    pub fn flat_map<F, R>(self, mapper: F) -> ParseResult<R>
    where
        F: FnOnce(T) -> ParseResult<R>,
    {
        match self {
            ParseResult::Success(value) => mapper(value),
            ParseResult::Fail(val) => ParseResult::Fail(val),
        }
    }

    pub fn or<F>(self, or_function: F) -> ParseResult<T>
    where
        F: FnOnce() -> ParseResult<T>,
    {
        match &self {
            ParseResult::Success(..) => self,
            _ => or_function(),
        }
    }
}
