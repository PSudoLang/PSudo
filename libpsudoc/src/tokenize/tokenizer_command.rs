use crate::coretypes::TokenCategory;
use crate::tokenize::RuleCategory;

type ToConsume = bool;

#[derive(Debug)]
pub enum TokenizerCommand {
    Continue(RuleCategory, ToConsume),
    Emit(TokenCategory, ToConsume),
    MoveCursorPrevious,
    Ignore(ToConsume),
}

impl TokenizerCommand {
    pub fn to_consume(&self) -> ToConsume {
        match self {
            TokenizerCommand::Continue(_, it) => *it,
            TokenizerCommand::Emit(_, it) => *it,
            TokenizerCommand::MoveCursorPrevious => false,
            TokenizerCommand::Ignore(it) => *it,
        }
    }
}
