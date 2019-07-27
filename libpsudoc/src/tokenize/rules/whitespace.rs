use crate::coretypes::{CodeCharacter, CodeCharacterCategory, TokenCategory};
use crate::tokenize::rules::Rule;
use crate::tokenize::{RuleCategory, TokenizerCommand};

pub struct RuleWhitespace;

impl Rule for RuleWhitespace {
    fn process(character: &CodeCharacter, _: &[CodeCharacter]) -> TokenizerCommand {
        match character.category {
            CodeCharacterCategory::HorizontalSpace => {
                TokenizerCommand::Continue(RuleCategory::Whitespace, true)
            }
            _ => TokenizerCommand::Emit(TokenCategory::Whitespace, false),
        }
    }
}
