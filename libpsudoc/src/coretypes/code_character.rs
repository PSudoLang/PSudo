#[derive(Debug, Clone)]
pub enum CodeCharacterCategory {
    HorizontalSpace,
    VerticalSpace,
    Identifiable,
    TogglingQuote,
    DecimalDigit,
    Punctuation,
    GroupOpeningCharacter,
    GroupClosingCharacter,
    EOF,
}

#[derive(Debug, Clone)]
pub struct CodeCharacter {
    pub data: char,
    pub category: CodeCharacterCategory,
}

impl CodeCharacter {
    pub const EOF: CodeCharacter = CodeCharacter {
        data: '\0',
        category: CodeCharacterCategory::EOF,
    };

    pub fn new(data: char) -> CodeCharacter {
        let category = match data {
            '0'..='9' => CodeCharacterCategory::DecimalDigit,
            | '\t'       // CHARACTER TABULATION
            | ' '        // SPACE
            | '\u{00A0}' // NO-BREAK SPACE
            | '\u{200E}' // LEFT-TO-RIGHT MARK
            | '\u{200F}' // RIGHT-TO-LEFT MARK
            | '\u{FEFF}' // ZERO WIDTH NO-BREAK SPACE
                => CodeCharacterCategory::HorizontalSpace,
            | '\n'       // LINE FEED
            | '\u{000B}' // LINE TABULATION
            | '\u{000C}' // FORM FEED
            | '\r'       // CARRIAGE RETURN
            | '\u{0085}' // NEXT LINE
            | '\u{2028}' // LINE SEPARATOR
            | '\u{2029}' // PARAGRAPH SEPARATOR 
                => CodeCharacterCategory::VerticalSpace,
            | '('        // LEFT PARENTHESIS
            | '['        // LEFT SQUARE BRACKET
            | '{'        // LEFT CURLY BRACKET
                => CodeCharacterCategory::GroupOpeningCharacter,
            | ')' // RIGHT PARENTHESIS
            | ']' // RIGHT SQUARE BRACKET
            | '}' // RIGHT CURLY BRACKET
                => CodeCharacterCategory::GroupClosingCharacter,
            | '"'  // QUOTATION MARK
            | '\'' // APOSTROPHE
            | '`'  // GRAVE ACCENT
                => CodeCharacterCategory::TogglingQuote,
            | '!'  // EXCLAMATION MARK
            | '#'  // NUMBER SIGN
            | '$'  // DOLLAR SIGN
            | '%'  // PERCENT SIGN
            | '&'  // AMPERSAND
            | '*'  // ASTREISK
            | '+'  // PLUS SIGN
            | ','  // COMMA
            | '-'  // HYPHEN-MINUS
            | '.'  // FULL STOP
            | '/'  // SOLIDUS
            | ':'  // COLON
            | ';'  // SEMICOLON
            | '<'  // LESS-THAN SIGN
            | '='  // EQUALS SIGN
            | '>'  // GREATER-THAN SIGN
            | '?'  // QUESTION MARK
            | '@'  // COMMERCIAL AT
            | '\\' // REVERSE SOLIDUS
            | '^'  // CIRCUMFLEX ACCENT
            | '|'  // VERTICAL LINE
            | '~'  // TILDE
                => CodeCharacterCategory::Punctuation,
            _ => CodeCharacterCategory::Identifiable,
        };

        CodeCharacter { data, category }
    }
}
