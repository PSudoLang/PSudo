use super::*;

use crate::coretypes::Node;

pub struct NodeStatement;

impl ParseFunction for NodeStatement {
    type Output = Node;
    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        Statement::try_parse(context, session).map(Node::Statement)
    }
}
