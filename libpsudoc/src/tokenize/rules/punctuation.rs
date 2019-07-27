use crate::coretypes::{CodeCharacter, CodeCharacterCategory, TokenCategory};
use crate::tokenize::rules::Rule;
use crate::tokenize::{RuleCategory, TokenizerCommand};

const MULTI_CHARACTER_PUNCTUATIONS: &[(&'static str, TokenCategory)] = &[
    ("&&", TokenCategory::Punctuation),
    ("||", TokenCategory::Punctuation),
    ("==", TokenCategory::Punctuation),
    ("!=", TokenCategory::Punctuation),
    ("<=>", TokenCategory::Punctuation),
    ("<=", TokenCategory::Punctuation),
    ("<<", TokenCategory::Punctuation),
    (">=", TokenCategory::Punctuation),
    (">>", TokenCategory::Punctuation),
    ("//", TokenCategory::LineCommentStart),
    ("/*", TokenCategory::BlockCommentStart),
    ("*/", TokenCategory::BlockCommentEnd),
    ("**", TokenCategory::Punctuation),
    ("->", TokenCategory::Punctuation),
    ("--", TokenCategory::Punctuation),
    ("++", TokenCategory::Punctuation),
];

pub struct RulePunctuation;

impl Rule for RulePunctuation {
    fn process(character: &CodeCharacter, characters: &[CodeCharacter]) -> TokenizerCommand {
        match character.category {
            CodeCharacterCategory::Punctuation if !characters.is_empty() => {
                for punctuation_set in MULTI_CHARACTER_PUNCTUATIONS {
                    let operator: String = [characters, &[character.clone()]]
                        .concat()
                        .iter()
                        .map(|character| character.data)
                        .collect();
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
