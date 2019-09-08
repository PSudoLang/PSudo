use crate::coretypes::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenCategory {
    // Group: Separator
    SeparatorLineWrap,

    // Group: Punctuation
    /// Punctuation `!`
    PunctuationExclamationMark,
    /// Punctuation `#`
    PunctuationNumberSign,
    /// Punctuation `$`
    PunctuationDollarSign,
    /// Punctuation `%`
    PunctuationPercentSign,
    /// Punctuation `&`
    PunctuationAmpersand,
    /// Punctuation `*`
    PunctuationAsterisk,
    /// Punctuation `+`
    PunctuationPlusSign,
    /// Punctuation `,`
    PunctuationComma,
    /// Punctuation `-`
    PunctuationHyphenMinus,
    /// Punctuation `.`
    PunctuationFullStop,
    /// Punctuation `/`
    PunctuationSolidus,
    /// Punctuation `:`
    PunctuationColon,
    /// Punctuation `;`
    PunctuationSemicolon,
    /// Punctuation `<`
    PunctuationLessThanSign,
    /// Punctuation `=`
    PunctuationEqualsSign,
    /// Punctuation `>`
    PunctuationGreaterThanSign,
    /// Punctuation `?`
    PunctuationQuestionMark,
    /// Punctuation `@`
    PunctuationCommercialAt,
    /// Punctuation `\`
    PunctuationReverseSolidus,
    /// Punctuation `^`
    PunctuationCircumflexAccent,
    /// Punctuation `|`
    PunctuationVerticalLine,
    /// Punctuation `~`
    PunctuationTilde,
    /// Punctuation `(`
    PunctuationLeftParenthesis,
    /// Punctuation `)`
    PunctuationRightParenthesis,
    /// Punctuation `[`
    PunctuationLeftSquareBracket,
    /// Punctuation `]`
    PunctuationRightSquareBracket,
    /// Punctuation `{`
    PunctuationLeftCurlyBracket,
    /// Punctuation `}`
    PunctuationRightCurlyBracket,
    /// Punctuations `&&`
    PunctuationsLogicalAnd,
    /// Punctuations `||`
    PunctuationsLogicalOr,
    /// Punctuations `==`
    PunctuationsEqualTo,
    /// Punctuations `!=`
    PunctuationsNotEqualTo,
    /// Punctuations `<=>`
    PunctuationsThreeWayComparison,
    /// Punctuations `<=`
    PunctuationsLessThanOrEqualTo,
    /// Punctuations `>=`
    PunctuationsGreaterThanOrEqualTo,
    /// Punctuations `<<`
    PunctuationsBitwiseLeftShift,
    /// Punctuations `>>`
    PunctuationsBitwiseRightShift,
    /// Punctuations `**`
    PunctuationsPow,
    /// Punctuations `->`
    PunctuationsSingleRightArrow,
    /// Punctuations `--`
    PunctuationsDecrement,
    /// Punctuations `++`
    PunctuationsIncrement,
    /// Punctuations `..`
    PunctuationsRangeInclusive,
    /// Punctuations `..<`
    PunctuationsRangeExclusive,

    // Group: Keyword
    Keyword, // TODO: Separating

    // Group: Identifier
    IdentifierIdentifier,
    IdentifierPlaceholderName,

    // Group: Literal
    LiteralString,
    LiteralStringNotEnded,
    LiteralNumber,
    LiteralBoolean,

    // Group: Comment
    CommentLine,
    CommentBlock,
    CommentBlockNotEnded,

    // Group: None
    Whitespace,
    Unknown,
}

impl TokenCategory {
    pub fn can_be_separator(&self) -> bool {
        match self {
            TokenCategory::SeparatorLineWrap
            | TokenCategory::PunctuationSemicolon
            | TokenCategory::CommentLine => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub category: TokenCategory,
    pub span: Span,
}
