mod parse_context;
mod parse_result;
mod rules;
mod util;

pub use parse_context::*;
pub use parse_result::*;
pub use rules::*;
pub use util::*;

pub use crate::coretypes::CompileSession;

pub trait ParseFunction {
    type Output;
    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output>;
}
