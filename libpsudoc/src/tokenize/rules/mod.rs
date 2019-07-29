mod comment;
mod identifier;
mod initial;
mod newline_cr;
mod number;
mod punctuation;
mod quoted_string;
mod whitespace;

use crate::coretypes::CodeCharacter;
use crate::tokenize::TokenizerCommand;

pub use comment::*;
pub use identifier::*;
pub use initial::*;
pub use newline_cr::*;
pub use number::*;
pub use punctuation::*;
pub use quoted_string::*;
pub use whitespace::*;

pub trait Rule {
    fn process(character: &CodeCharacter, characters: &[CodeCharacter]) -> TokenizerCommand;
}
