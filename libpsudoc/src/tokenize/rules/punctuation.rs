use super::Rule;
use crate::coretypes::{CodeCharacter, CodeCharacterCategory, TokenCategory};
use crate::tokenize::{RuleCategory, TokenizerCommand};

const MULTI_CHARACTER_PUNCTUATIONS: &[(&str, TokenCategory)] = &[
    ("&&", TokenCategory::Punctuation),
    ("||", TokenCategory::Punctuation),
    ("==", TokenCategory::Punctuation),
    ("!=", TokenCategory::Punctuation),
    ("<=>", TokenCategory::Punctuation),
    ("<=", TokenCategory::Punctuation),
    ("<<", TokenCategory::Punctuation),
    (">=", TokenCategory::Punctuation),
    (">>", TokenCategory::Punctuation),
    ("**", TokenCategory::Punctuation),
    ("->", TokenCategory::Punctuation),
    ("--", TokenCategory::Punctuation),
    ("++", TokenCategory::Punctuation),
    ("..", TokenCategory::Punctuation),
    ("..<", TokenCategory::Punctuation),
];

pub struct RulePunctuation;

impl Rule for RulePunctuation {
    fn process(character: &CodeCharacter, characters: &[CodeCharacter]) -> TokenizerCommand {
        match character.category {
            CodeCharacterCategory::Punctuation if !characters.is_empty() => {
                let operator: String = [characters, &[character.clone()]]
                    .concat()
                    .iter()
                    .map(|character| character.data)
                    .collect();
                if operator == "//" {
                    return TokenizerCommand::Continue(RuleCategory::LineComment, false);
                }
                if operator == "/*" {
                    return TokenizerCommand::Continue(RuleCategory::BlockComment, false);
                }
                for punctuation_set in MULTI_CHARACTER_PUNCTUATIONS {
                    if punctuation_set.0.starts_with(&operator) {
                        return TokenizerCommand::Continue(RuleCategory::Punctuation, true);
                    }
                    if operator == punctuation_set.0 {
                        return TokenizerCommand::Emit(punctuation_set.1.clone(), true);
                    }
                }
                TokenizerCommand::Emit(TokenCategory::Punctuation, false)
            }
            _ => TokenizerCommand::Emit(TokenCategory::Punctuation, false),
        }
    }
}
