use super::Rule;
use crate::coretypes::{CodeCharacter, CodeCharacterCategory, TokenCategory};
use crate::tokenize::{RuleCategory, TokenizerCommand};

const KEYWORDS: &[&str] = &[
    "as", "break", "continue", "const", "else", "enum", "extern", "fn", "for", "if", "impl", "in",
    "let", "loop", "match", "move", "ref", "return", "self", "Self", "static", "struct", "super",
    "trait", "type", "where", "while", "input", "output", "log",
];

const BOOL: &[&str] = &["false", "true"];

pub struct RuleIdentifier;

impl Rule for RuleIdentifier {
    fn process(character: &CodeCharacter, characters: &[CodeCharacter]) -> TokenizerCommand {
        match character.category {
            CodeCharacterCategory::Identifiable => {
                TokenizerCommand::Continue(RuleCategory::Identifier, true)
            }
            CodeCharacterCategory::DecimalDigit => {
                TokenizerCommand::Continue(RuleCategory::Identifier, true)
            }
            _ => {
                let code_string = characters.iter().map(|it| it.data).collect::<String>();
                TokenizerCommand::Emit(
                    if characters.len() == 1 && characters[0].data == '_' {
                        TokenCategory::IdentifierPlaceholderName
                    } else if KEYWORDS.contains(&code_string.as_str()) {
                        TokenCategory::Keyword
                    } else if BOOL.contains(&code_string.as_str()) {
                        TokenCategory::LiteralBoolean
                    } else {
                        TokenCategory::IdentifierIdentifier
                    },
                    false,
                )
            }
        }
    }
}
