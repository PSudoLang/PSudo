use super::Rule;
use crate::coretypes::{CodeCharacter, CodeCharacterCategory, TokenCategory};
use crate::tokenize::{RuleCategory, TokenizerCommand};

pub struct RuleInitial;

impl Rule for RuleInitial {
    fn process(character: &CodeCharacter, _: &[CodeCharacter]) -> TokenizerCommand {
        match character.category {
            CodeCharacterCategory::HorizontalSpace => {
                TokenizerCommand::Continue(RuleCategory::Whitespace, true)
            }
            CodeCharacterCategory::VerticalSpace => {
                if character.data == '\r' {
                    TokenizerCommand::Continue(RuleCategory::NewlineCr, true)
                } else {
                    TokenizerCommand::Emit(TokenCategory::SeparatorLineWrap, true)
                }
            }
            CodeCharacterCategory::Punctuation => {
                TokenizerCommand::Continue(RuleCategory::Punctuation, true)
            }
            CodeCharacterCategory::Identifiable => {
                TokenizerCommand::Continue(RuleCategory::Identifier, true)
            }
            CodeCharacterCategory::TogglingQuote => {
                TokenizerCommand::Continue(RuleCategory::QuotedString, true)
            }
            CodeCharacterCategory::DecimalDigit => {
                TokenizerCommand::Continue(RuleCategory::Number, true)
            }
            CodeCharacterCategory::EOF => TokenizerCommand::Ignore(true),
        }
    }
}
