use crate::coretypes::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenCategory {
    Whitespace,
    LineWrap,
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
    GroupOpen,
    GroupClose,
    Unknown,
}

impl TokenCategory {
    pub fn can_be_separator(&self) -> bool {
        match self {
            TokenCategory::LineWrap | TokenCategory::LineComment | TokenCategory::Separator => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub category: TokenCategory,
    pub span: Span,
}
