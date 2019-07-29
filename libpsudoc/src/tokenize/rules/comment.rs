use super::Rule;
use crate::coretypes::{CodeCharacter, CodeCharacterCategory, TokenCategory};
use crate::tokenize::{RuleCategory, TokenizerCommand};

pub struct RuleLineComment;

impl Rule for RuleLineComment {
    fn process(character: &CodeCharacter, _: &[CodeCharacter]) -> TokenizerCommand {
        match character.category {
            CodeCharacterCategory::VerticalSpace | CodeCharacterCategory::EOF => {
                TokenizerCommand::Emit(TokenCategory::LineComment, false)
            }
            _ => TokenizerCommand::Continue(RuleCategory::LineComment, true),
        }
    }
}

pub struct RuleBlockComment;

impl Rule for RuleBlockComment {
    fn process(character: &CodeCharacter, characters: &[CodeCharacter]) -> TokenizerCommand {
        match character.category {
            CodeCharacterCategory::Punctuation
                if character.data == '/' && characters[characters.len() - 1].data == '*' =>
            {
                TokenizerCommand::Emit(TokenCategory::BlockComment, true)
            }
            CodeCharacterCategory::EOF => {
                TokenizerCommand::Emit(TokenCategory::NotEndedBlockComment, false)
            }
            _ => TokenizerCommand::Continue(RuleCategory::BlockComment, true),
        }
    }
}
