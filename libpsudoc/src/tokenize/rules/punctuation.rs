use super::Rule;
use crate::coretypes::{CodeCharacter, CodeCharacterCategory, TokenCategory};
use crate::tokenize::{RuleCategory, TokenizerCommand};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref PUNCTUATION_MAP: HashMap<&'static str, TokenCategory> = {
        let mut map = HashMap::new();
        map.insert("!", TokenCategory::PunctuationExclamationMark);
        map.insert("#", TokenCategory::PunctuationNumberSign);
        map.insert("$", TokenCategory::PunctuationDollarSign);
        map.insert("%", TokenCategory::PunctuationPercentSign);
        map.insert("&", TokenCategory::PunctuationAmpersand);
        map.insert("*", TokenCategory::PunctuationAsterisk);
        map.insert("+", TokenCategory::PunctuationPlusSign);
        map.insert(",", TokenCategory::PunctuationComma);
        map.insert("-", TokenCategory::PunctuationHyphenMinus);
        map.insert(".", TokenCategory::PunctuationFullStop);
        map.insert("/", TokenCategory::PunctuationSolidus);
        map.insert(":", TokenCategory::PunctuationColon);
        map.insert(";", TokenCategory::PunctuationSemicolon);
        map.insert("<", TokenCategory::PunctuationLessThanSign);
        map.insert("=", TokenCategory::PunctuationEqualsSign);
        map.insert(">", TokenCategory::PunctuationGreaterThanSign);
        map.insert("?", TokenCategory::PunctuationQuestionMark);
        map.insert("@", TokenCategory::PunctuationCommercialAt);
        map.insert("\\", TokenCategory::PunctuationReverseSolidus);
        map.insert("^", TokenCategory::PunctuationCircumflexAccent);
        map.insert("|", TokenCategory::PunctuationVerticalLine);
        map.insert("~", TokenCategory::PunctuationTilde);
        map.insert("(", TokenCategory::PunctuationLeftParenthesis);
        map.insert("[", TokenCategory::PunctuationLeftSquareBracket);
        map.insert("{", TokenCategory::PunctuationLeftCurlyBracket);
        map.insert(")", TokenCategory::PunctuationRightParenthesis);
        map.insert("]", TokenCategory::PunctuationRightSquareBracket);
        map.insert("}", TokenCategory::PunctuationRightCurlyBracket);
        map.insert("&&", TokenCategory::PunctuationsLogicalAnd);
        map.insert("||", TokenCategory::PunctuationsLogicalOr);
        map.insert("==", TokenCategory::PunctuationsEqualTo);
        map.insert("!=", TokenCategory::PunctuationsNotEqualTo);
        map.insert("<=>", TokenCategory::PunctuationsThreeWayComparison);
        map.insert("<=", TokenCategory::PunctuationsLessThanOrEqualTo);
        map.insert(">=", TokenCategory::PunctuationsGreaterThanOrEqualTo);
        map.insert("<<", TokenCategory::PunctuationsBitwiseLeftShift);
        map.insert(">>", TokenCategory::PunctuationsBitwiseRightShift);
        map.insert("**", TokenCategory::PunctuationsPow);
        map.insert("->", TokenCategory::PunctuationsSingleRightArrow);
        map.insert("--", TokenCategory::PunctuationsDecrement);
        map.insert("++", TokenCategory::PunctuationsIncrement);
        map.insert("..", TokenCategory::PunctuationsRangeInclusive);
        map.insert("..<", TokenCategory::PunctuationsRangeExclusive);
        map
    };
}

pub struct RulePunctuation;

impl Rule for RulePunctuation {
    fn process(character: &CodeCharacter, characters: &[CodeCharacter]) -> TokenizerCommand {
        match character.category {
            CodeCharacterCategory::Punctuation if !characters.is_empty() => {
                let punctuation_joined: String = [characters, &[character.clone()]]
                    .concat()
                    .iter()
                    .map(|character| character.data)
                    .collect();
                match punctuation_joined.as_str() {
                    "//" => TokenizerCommand::Continue(RuleCategory::LineComment, true),
                    "/*" => TokenizerCommand::Continue(RuleCategory::BlockComment, true),
                    _ => {
                        for (punctuation, token_category) in PUNCTUATION_MAP.iter() {
                            if punctuation.starts_with(&punctuation_joined) {
                                return TokenizerCommand::Continue(RuleCategory::Punctuation, true);
                            }
                            if &punctuation_joined == punctuation {
                                return TokenizerCommand::Emit(token_category.clone(), true);
                            }
                        }
                        TokenizerCommand::Emit(
                            PUNCTUATION_MAP
                                .get(
                                    &characters
                                        .iter()
                                        .map(|character| character.data)
                                        .collect::<String>()
                                        .as_str(),
                                )
                                .expect("Guaranteed by rule")
                                .clone(),
                            false,
                        )
                    }
                }
            }
            _ => TokenizerCommand::Emit(
                PUNCTUATION_MAP
                    .get(
                        &characters
                            .iter()
                            .map(|character| character.data)
                            .collect::<String>()
                            .as_str(),
                    )
                    .expect("Guaranteed by rule")
                    .clone(),
                false,
            ),
        }
    }
}
