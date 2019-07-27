#[derive(Debug)]
pub enum RuleCategory {
    Initial,
    Whitespace,
    NewlineCr,
    Identifier,
    QuotedString,
    Number,
    NumberExponent,
    NumberDecimal,
    Punctuation,
}
