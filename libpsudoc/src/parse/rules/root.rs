use super::*;
use crate::coretypes::Node;

pub struct Root;

impl ParseFunction for Root {
    type Output = Node;
    fn try_parse(
        context: &mut ParseContext,
        session: &mut CompileSession,
    ) -> ParseResult<Self::Output> {
        let mut nodes = Vec::new();

        let mut failed = false;
        context.skip_whitespaces(true);

        while context.has_next() {
            match try_all(
                &[
                    LineComment::try_parse,
                    BlockComment::try_parse,
                    NodeStatement::try_parse,
                ],
                context,
                session,
            ) {
                ParseResult::Success(node) => {
                    nodes.push(node);
                }
                ParseResult::Fail(val) => {
                    if !val {
                        context
                            .last_read_token()
                            .span
                            .diagnostic_error("Unexpected token")
                            .emit_to(session);
                        return ParseResult::Fail(true);
                    }

                    failed = true;
                }
            }
            context.skip_whitespaces(true);
        }

        if failed {
            ParseResult::Fail(true)
        } else {
            ParseResult::Success(Node::Root(nodes))
        }
    }
}
