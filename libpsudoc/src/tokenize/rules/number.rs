use super::Rule;
use crate::coretypes::{CodeCharacter, CodeCharacterCategory, TokenCategory};
use crate::tokenize::{RuleCategory, TokenizerCommand};

#[derive(PartialEq)]
enum LiteralType {
    Hexadecimal,
    Octal,
    Decimal,
    Binary,
    Unknown,
}

fn get_literal_type(characters: &[CodeCharacter]) -> LiteralType {
    if characters.len() < 2 {
        return LiteralType::Unknown;
    }
    if characters[0].data != '0' {
        return LiteralType::Decimal;
    }
    match characters[1].data {
        'x' | 'X' => LiteralType::Hexadecimal,
        'o' | 'O' => LiteralType::Octal,
        'b' | 'B' => LiteralType::Binary,
        _ => LiteralType::Decimal,
    }
}

fn is_hexadecimal_part(c: char) -> bool {
    match c {
        'a'..='f' | 'A'..='F' => true,
        _ => false,
    }
}

pub struct RuleNumber;

impl Rule for RuleNumber {
    fn process(character: &CodeCharacter, characters: &[CodeCharacter]) -> TokenizerCommand {
        match character.category {
            CodeCharacterCategory::Punctuation if character.data == '.' => {
                TokenizerCommand::Continue(RuleCategory::NumberDecimal, true)
            }
            CodeCharacterCategory::Identifiable => {
                if get_literal_type(characters) == LiteralType::Hexadecimal
                    && is_hexadecimal_part(character.data)
                {
                    TokenizerCommand::Continue(RuleCategory::Number, true)
                } else if characters.len() == 1 && characters[0].data == '0' {
                    match character.data {
                        'x' | 'X' | 'o' | 'O' | 'b' | 'B' => {
                            TokenizerCommand::Continue(RuleCategory::Number, true)
                        }
                        _ => TokenizerCommand::Emit(TokenCategory::LiteralNumber, false),
                    }
                } else if character.data == 'e' {
                    TokenizerCommand::Continue(RuleCategory::NumberExponent, true)
                } else {
                    TokenizerCommand::Emit(TokenCategory::LiteralNumber, false)
                }
            }
            CodeCharacterCategory::DecimalDigit => {
                TokenizerCommand::Continue(RuleCategory::Number, true)
            }
            _ => TokenizerCommand::Emit(TokenCategory::LiteralNumber, false),
        }
    }
}

pub struct RuleNumberDecimal;

impl Rule for RuleNumberDecimal {
    fn process(character: &CodeCharacter, characters: &[CodeCharacter]) -> TokenizerCommand {
        match character.category {
            CodeCharacterCategory::Identifiable if character.data == 'e' => {
                if characters
                    .last()
                    .map(|code_character| code_character.data == '.')
                    .unwrap_or(false)
                {
                    TokenizerCommand::MoveCursorPrevious
                } else {
                    TokenizerCommand::Continue(RuleCategory::NumberExponent, true)
                }
            }
            CodeCharacterCategory::DecimalDigit => {
                TokenizerCommand::Continue(RuleCategory::NumberDecimal, true)
            }
            _ => {
                if characters
                    .last()
                    .map(|code_character| code_character.data == '.')
                    .unwrap_or(false)
                {
                    TokenizerCommand::MoveCursorPrevious
                } else {
                    TokenizerCommand::Emit(TokenCategory::LiteralNumber, false)
                }
            }
        }
    }
}

pub struct RuleNumberExponent;

impl Rule for RuleNumberExponent {
    fn process(character: &CodeCharacter, characters: &[CodeCharacter]) -> TokenizerCommand {
        match character.category {
            CodeCharacterCategory::DecimalDigit => {
                TokenizerCommand::Continue(RuleCategory::NumberExponent, true)
            }
            _ => {
                if characters
                    .last()
                    .map(|code_character| code_character.data == 'e' || code_character.data == 'E')
                    .unwrap_or(false)
                {
                    TokenizerCommand::MoveCursorPrevious
                } else {
                    TokenizerCommand::Emit(TokenCategory::LiteralNumber, false)
                }
            }
        }
    }
}
