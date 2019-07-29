use crate::coretypes::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenCategory {
    Whitespace,
    Separator,
    Punctuation,
    Keyword,
    Identifier,
    PlaceholderName,
    LiteralString,
    NotEndedLiteralString,
    LiteralNumber,
    LiteralBoolean,
    LineComment,
    BlockComment,
    NotEndedBlockComment,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub category: TokenCategory,
    pub span: Span,
}
