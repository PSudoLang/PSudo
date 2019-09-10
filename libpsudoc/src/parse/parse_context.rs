use crate::coretypes::{Token, TokenCategory};

pub struct ParseContext {
    tokens: Vec<Token>,
    pub current: usize,
    /// Current operator precedence
    ///
    /// | level              | for                              |
    /// | ------------------ | -------------------------------- |
    /// | 1                  | `*`, `/`, `%`                    |
    /// | 2                  | `+`, `-`                         |
    /// | 3                  | `<<`, `>>`                       |
    /// | 4                  | `&`                              |
    /// | 5                  | `^`                              |
    /// | 6                  | `|`                              |
    /// | 7                  | `==`, `!=`, `<`, `>`, `<=`, `>=` |
    /// | 8                  | `&&`                             |
    /// | 9                  | `||`                             |
    /// | 10                 | `..`, `..<`                      |
    /// | usize::max_value() | no-operators                     |
    pub operator_precedence: usize,
}

impl ParseContext {
    pub fn new(tokens: Vec<Token>) -> ParseContext {
        ParseContext {
            tokens,
            current: 0,
            operator_precedence: usize::max_value(),
        }
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

    pub fn last_read_token(&self) -> &Token {
        &self.tokens[std::cmp::max(1, std::cmp::min(self.tokens.len(), self.current)) - 1]
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

    pub fn next_token_categoried(&mut self, categories: &[TokenCategory]) -> Option<&Token> {
        self.next_if_matched(|token| categories.contains(&token.category))
    }

    pub fn next_if_matched<F>(&mut self, predicate: F) -> Option<&Token>
    where
        F: Fn(&Token) -> bool,
    {
        if self.has_next() {
            let next = &self.tokens[self.current];
            if predicate(next) {
                self.current += 1;
                Some(next)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn skip_whitespaces(&mut self, skip_linefeeds: bool) -> bool {
        let mut skipped = false;
        while let Some(token) = self.peek() {
            if token.category != TokenCategory::Whitespace
                && (!skip_linefeeds || token.category != TokenCategory::SeparatorLineWrap)
            {
                break;
            }
            self.next();
            skipped = true;
        }
        skipped
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
            operator_precedence: self.operator_precedence,
        }
    }
}
