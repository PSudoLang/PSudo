use crate::coretypes::Span;

#[derive(Debug, Clone)]
pub enum TokenCategory {
    Whitespace,
    Separator,
    Punctuation,
    Keyword,
    Identifier,
    PlaceholderName,
    LiteralString,
    LiteralNumber,
    LiteralBoolean,
    LineCommentStart,
    BlockCommentStart,
    BlockCommentEnd,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub category: TokenCategory,
    pub span: Span,
}
