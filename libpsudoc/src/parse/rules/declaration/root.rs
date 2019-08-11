use super::*;
use crate::coretypes::Declaration as DeclarationNode;

pub struct Declaration;

impl ParseFunction for Declaration {
    type Output = DeclarationNode;
    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        try_all(
            vec![
                Function::try_parse,
            ],
            context,
            session,
        )
    }
}
