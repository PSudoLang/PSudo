use super::Rule;
use crate::coretypes::{CodeCharacter, CodeCharacterCategory, TokenCategory};
use crate::tokenize::{RuleCategory, TokenizerCommand};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenCategory> = {
        let mut map = HashMap::new();
        map.insert("as", TokenCategory::KeywordAs);
        map.insert("break", TokenCategory::KeywordBreak);
        map.insert("continue", TokenCategory::KeywordContinue);
        map.insert("const", TokenCategory::KeywordConst);
        map.insert("else", TokenCategory::KeywordElse);
        map.insert("enum", TokenCategory::KeywordEnum);
        map.insert("extern", TokenCategory::KeywordExtern);
        map.insert("fn", TokenCategory::KeywordFn);
        map.insert("for", TokenCategory::KeywordFor);
        map.insert("if", TokenCategory::KeywordIf);
        map.insert("impl", TokenCategory::KeywordImpl);
        map.insert("in", TokenCategory::KeywordIn);
        map.insert("let", TokenCategory::KeywordLet);
        map.insert("loop", TokenCategory::KeywordLoop);
        map.insert("match", TokenCategory::KeywordMatch);
        map.insert("move", TokenCategory::KeywordMove);
        map.insert("ref", TokenCategory::KeywordRef);
        map.insert("return", TokenCategory::KeywordReturn);
        map.insert("self", TokenCategory::KeywordSelf);
        map.insert("Self", TokenCategory::KeywordSelfType);
        map.insert("static", TokenCategory::KeywordStatic);
        map.insert("struct", TokenCategory::KeywordStruct);
        map.insert("super", TokenCategory::KeywordSuper);
        map.insert("trait", TokenCategory::KeywordTrait);
        map.insert("type", TokenCategory::KeywordType);
        map.insert("where", TokenCategory::KeywordWhere);
        map.insert("while", TokenCategory::KeywordWhile);
        map.insert("input", TokenCategory::KeywordInput);
        map.insert("output", TokenCategory::KeywordOutput);
        map.insert("log", TokenCategory::KeywordLog);
        map
    };
}

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
                    } else if let Some(category) = KEYWORDS.get(&code_string.as_str()) {
                        category.clone()
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
