use super::Rule;
use crate::coretypes::{CodeCharacter, CodeCharacterCategory, TokenCategory};
use crate::tokenize::{RuleCategory, TokenizerCommand};

pub struct RuleNewlineCr;

impl Rule for RuleNewlineCr {
    fn process(character: &CodeCharacter, _: &[CodeCharacter]) -> TokenizerCommand {
        match character.category {
            CodeCharacterCategory::VerticalSpace if character.data == '\r' => {
                TokenizerCommand::Continue(RuleCategory::NewlineCr, character.data == '\n')
            }
            _ => TokenizerCommand::Emit(TokenCategory::LineWrap, false),
        }
    }
}
