use super::Rule;
use crate::coretypes::{CodeCharacter, CodeCharacterCategory, TokenCategory};
use crate::tokenize::{RuleCategory, TokenizerCommand};

pub struct RuleQuotedString;

impl Rule for RuleQuotedString {
    fn process(character: &CodeCharacter, characters: &[CodeCharacter]) -> TokenizerCommand {
        match character.category {
            CodeCharacterCategory::TogglingQuote
                if !characters.is_empty()
                    && character.data == characters[0].data
                    && characters[characters.len() - 1].data != '\\' =>
            {
                TokenizerCommand::Emit(TokenCategory::LiteralString, true)
            }
            CodeCharacterCategory::EOF => {
                TokenizerCommand::Emit(TokenCategory::NotEndedLiteralString, false)
            }
            _ => TokenizerCommand::Continue(RuleCategory::QuotedString, true),
        }
    }
}
